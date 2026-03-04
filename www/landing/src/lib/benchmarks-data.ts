export interface BenchmarkMetric {
  label: string;
  justkv: number;
  redis: string | number;
  unit: string;
  description: string;
}

export const keyMetrics: BenchmarkMetric[] = [
  {
    label: "SET Operations",
    justkv: 312000,
    redis: 245000,
    unit: "ops/sec",
    description: "Single key SET operations per second",
  },
  {
    label: "GET Operations",
    justkv: 398000,
    redis: 310000,
    unit: "ops/sec",
    description: "Single key GET operations per second",
  },
  {
    label: "Memory Usage",
    justkv: 1.2,
    redis: 1.8,
    unit: "GB",
    description: "Memory used for 1M keys with 256-byte values",
  },
  {
    label: "p99 Latency",
    justkv: 0.28,
    redis: 0.41,
    unit: "ms",
    description: "99th percentile latency under load",
  },
];

export interface ThroughputData {
  operation: string;
  JustKV: number;
  Redis: number;
}

export const throughputData: ThroughputData[] = [
  { operation: "SET", JustKV: 312000, Redis: 245000 },
  { operation: "GET", JustKV: 398000, Redis: 310000 },
  { operation: "MSET (10)", JustKV: 185000, Redis: 142000 },
  { operation: "MGET (10)", JustKV: 220000, Redis: 175000 },
  { operation: "INCR", JustKV: 345000, Redis: 280000 },
  { operation: "LPUSH", JustKV: 295000, Redis: 230000 },
  { operation: "SADD", JustKV: 278000, Redis: 225000 },
  { operation: "HSET", JustKV: 265000, Redis: 215000 },
];

export interface MemoryData {
  dataset: string;
  JustKV: number;
  Redis: number;
}

export const memoryData: MemoryData[] = [
  { dataset: "100K keys", JustKV: 125, Redis: 182 },
  { dataset: "500K keys", JustKV: 610, Redis: 905 },
  { dataset: "1M keys", JustKV: 1200, Redis: 1800 },
  { dataset: "5M keys", JustKV: 5800, Redis: 8900 },
  { dataset: "10M keys", JustKV: 11200, Redis: 17600 },
];

export interface LatencyData {
  percentile: string;
  JustKV: number;
  Redis: number;
}

export const latencyData: LatencyData[] = [
  { percentile: "p50", JustKV: 0.08, Redis: 0.11 },
  { percentile: "p90", JustKV: 0.15, Redis: 0.22 },
  { percentile: "p95", JustKV: 0.19, Redis: 0.29 },
  { percentile: "p99", JustKV: 0.28, Redis: 0.41 },
  { percentile: "p99.9", JustKV: 0.52, Redis: 0.85 },
];

export const testEnvironment = {
  cpu: "AMD EPYC 7763 (8 vCPUs)",
  memory: "32 GB DDR4-3200",
  os: "Ubuntu 24.04 LTS",
  kernel: "6.8.0",
  network: "Loopback (localhost)",
  clients: "50 concurrent connections",
  pipeline: "16 commands per pipeline",
  dataSize: "256-byte values, random keys",
  justkvVersion: "0.1.0",
  redisVersion: "7.4.2",
};
