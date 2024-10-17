export const handler = async () => {
  const now = Date.now();
  const res = await fetch("http://localhost:8000");
  console.log(`fetch: ${Date.now() - now}ms`);

  return {
    statusCode: 200,
    body: await res.text(),
  };
};
