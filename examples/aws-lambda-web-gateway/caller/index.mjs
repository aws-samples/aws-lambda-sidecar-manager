export const handler = async () => {
  const res = await fetch("http://localhost:8000");

  return {
    statusCode: 200,
    body: await res.text(),
  };
};
