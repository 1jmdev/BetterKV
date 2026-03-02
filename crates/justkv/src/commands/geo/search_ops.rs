use crate::commands::geo::parse::{
    parse_distance_unit, parse_f64, parse_search_options, SearchOptions, SortOrder,
};
use crate::commands::util::{f64_to_bytes, wrong_args, wrong_type, Args};
use crate::engine::store::Store;
use crate::protocol::types::{BulkData, RespFrame};

pub(crate) fn geosearch(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 8 {
        return wrong_args("GEOSEARCH");
    }
    if !args[2].eq_ignore_ascii_case(b"FROMLONLAT") {
        return RespFrame::Error("ERR syntax error".to_string());
    }

    let lon = match parse_f64(&args[3]) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let lat = match parse_f64(&args[4]) {
        Ok(value) => value,
        Err(response) => return response,
    };
    run_geosearch(store, &args[1], (lon, lat), &args[5..])
}

pub(crate) fn geosearchstore(store: &Store, args: &Args) -> RespFrame {
    if args.len() < 10 {
        return wrong_args("GEOSEARCHSTORE");
    }

    let destination = &args[1];
    let source = &args[2];
    if !args[3].eq_ignore_ascii_case(b"FROMLONLAT") {
        return RespFrame::Error("ERR syntax error".to_string());
    }

    let lon = match parse_f64(&args[4]) {
        Ok(value) => value,
        Err(response) => return response,
    };
    let lat = match parse_f64(&args[5]) {
        Ok(value) => value,
        Err(response) => return response,
    };

    let mut options = match parse_geo_byargs(&args[6..]) {
        Ok(value) => value,
        Err(response) => return response,
    };

    let store_dist = options.storedist.take().is_some();
    if options.store.is_none() && !store_dist {
        options.store = Some(destination.to_vec());
    }

    run_store_search(store, destination, source, (lon, lat), options, store_dist)
}

pub(crate) fn run_radius_search(
    store: &Store,
    key: &[u8],
    center: (f64, f64),
    radius_meters: f64,
    options: SearchOptions,
) -> RespFrame {
    if let Some(destination) = options.store.clone().or(options.storedist.clone()) {
        return run_store_search(
            store,
            &destination,
            key,
            center,
            options,
            destination == options.storedist.unwrap_or_default(),
        );
    }

    let ascending = !matches!(options.sort, Some(SortOrder::Desc));
    match store.geosearch(
        key,
        center,
        Some(radius_meters),
        None,
        ascending,
        options.count,
    ) {
        Ok(matches) => format_matches(matches, options),
        Err(_) => wrong_type(),
    }
}

fn run_geosearch(
    store: &Store,
    key: &[u8],
    center: (f64, f64),
    args: &[crate::engine::value::CompactArg],
) -> RespFrame {
    let (radius, box_size, options) = match parse_shape_and_options(args) {
        Ok(value) => value,
        Err(response) => return response,
    };

    if let Some(destination) = options.store.clone().or(options.storedist.clone()) {
        return run_store_search(
            store,
            &destination,
            key,
            center,
            options,
            destination == options.storedist.unwrap_or_default(),
        );
    }

    let ascending = !matches!(options.sort, Some(SortOrder::Desc));
    match store.geosearch(key, center, radius, box_size, ascending, options.count) {
        Ok(matches) => format_matches(matches, options),
        Err(_) => wrong_type(),
    }
}

fn run_store_search(
    store: &Store,
    destination: &[u8],
    source: &[u8],
    center: (f64, f64),
    options: SearchOptions,
    store_dist: bool,
) -> RespFrame {
    let (radius, box_size) = match shape_from_options(&options) {
        Some(value) => value,
        None => return RespFrame::Error("ERR syntax error".to_string()),
    };

    let ascending = !matches!(options.sort, Some(SortOrder::Desc));
    match store.geosearchstore(
        destination,
        source,
        center,
        radius,
        box_size,
        ascending,
        options.count,
        store_dist,
    ) {
        Ok(value) => RespFrame::Integer(value),
        Err(_) => wrong_type(),
    }
}

fn parse_geo_byargs(args: &[crate::engine::value::CompactArg]) -> Result<SearchOptions, RespFrame> {
    let (_, _, mut options) = parse_shape_and_options(args)?;
    options.withcoord = false;
    options.withdist = false;
    options.withhash = false;
    Ok(options)
}

fn parse_shape_and_options(
    args: &[crate::engine::value::CompactArg],
) -> Result<(Option<f64>, Option<(f64, f64)>, SearchOptions), RespFrame> {
    if args.len() < 3 {
        return Err(RespFrame::Error("ERR syntax error".to_string()));
    }

    let mut radius = None;
    let mut box_size = None;
    let mut index = 0usize;
    if args[index].eq_ignore_ascii_case(b"BYRADIUS") {
        let value = parse_f64(&args[index + 1])?;
        let unit = parse_distance_unit(&args[index + 2])?;
        radius = Some(value * unit);
        index += 3;
    } else if args[index].eq_ignore_ascii_case(b"BYBOX") {
        let width = parse_f64(&args[index + 1])?;
        let height = parse_f64(&args[index + 2])?;
        let unit = parse_distance_unit(&args[index + 3])?;
        box_size = Some((width * unit, height * unit));
        index += 4;
    } else {
        return Err(RespFrame::Error("ERR syntax error".to_string()));
    }

    let options = parse_search_options(args, index)?;
    Ok((radius, box_size, options))
}

fn shape_from_options(options: &SearchOptions) -> Option<(Option<f64>, Option<(f64, f64)>)> {
    if options.withcoord || options.withdist || options.withhash || options.any {
        return Some((None, None));
    }
    Some((None, None))
}

fn format_matches(
    matches: Vec<crate::engine::store::geo::GeoSearchMatch>,
    options: SearchOptions,
) -> RespFrame {
    let out = matches
        .into_iter()
        .map(|entry| {
            if !options.withcoord && !options.withdist && !options.withhash {
                return RespFrame::Bulk(Some(BulkData::Arg(entry.member)));
            }

            let mut item = vec![RespFrame::Bulk(Some(BulkData::Arg(entry.member)))];
            if options.withdist {
                item.push(RespFrame::Bulk(Some(BulkData::from_vec(f64_to_bytes(
                    entry.distance_meters.unwrap_or(0.0),
                )))));
            }
            if options.withhash {
                item.push(RespFrame::Integer(0));
            }
            if options.withcoord {
                item.push(RespFrame::Array(Some(vec![
                    RespFrame::Bulk(Some(BulkData::from_vec(f64_to_bytes(entry.longitude)))),
                    RespFrame::Bulk(Some(BulkData::from_vec(f64_to_bytes(entry.latitude)))),
                ])));
            }
            RespFrame::Array(Some(item))
        })
        .collect();
    RespFrame::Array(Some(out))
}
