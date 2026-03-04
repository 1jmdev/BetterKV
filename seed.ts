import { RedisClient } from "bun";

const client = new RedisClient("redis://localhost:6379");

const TOTAL_ITEMS = 50_000_000;
const BATCH_SIZE = 1_000; // Number of key-value pairs per MSET

async function seed() {
  console.log(`Starting to seed ${TOTAL_ITEMS} items using MSET...`);
  const startTime = performance.now();

  for (let i = 0; i < TOTAL_ITEMS; i += BATCH_SIZE) {
    // Construct the flat array: [key1, val1, key2, val2, ...]
    const batch: string[] = [];

    for (let j = 0; j < BATCH_SIZE && i + j < TOTAL_ITEMS; j++) {
      const index = i + j;
      batch.push(`item:${index}`, `value_${index}`);
    }

    await client.mset(...batch);

    if (i % (BATCH_SIZE * 50) === 0) {
      const progress = ((i / TOTAL_ITEMS) * 100).toFixed(2);
      console.log(`Progress: ${progress}%`);
    }
  }

  const duration = ((performance.now() - startTime) / 1000).toFixed(2);
  console.log(`Seeding complete. Time taken: ${duration} seconds.`);
}

seed().catch((err) => {
  console.error("Seeding failed:", err);
  process.exit(1);
});