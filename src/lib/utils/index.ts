function rsplit(str: string, sep: string, limit?: number): string[] {
  if (limit === undefined || limit <= 0) {
    return str.split(sep);
  }

  const result: string[] = [];
  let remaining = str;

  for (let i = 0; i < limit; i++) {
    const idx = remaining.lastIndexOf(sep);
    if (idx === -1) {
      break;
    }
    result.unshift(remaining.slice(idx + sep.length));
    remaining = remaining.slice(0, idx);
  }
  result.unshift(remaining);

  return result;
}

function formatUrl(sites: string) {
  let lines = sites
    .trimStart()
    .trimEnd()
    .split("\n")
    ?.map((item) => {
      const parts = item.split(" ");
      const url = parts[parts.length - 1]; // Get last element (URL)
      const title = parts.slice(0, -1).join(" "); // Everything except last element (title)
      return {
        title: title || "No title",
        url: url || "",
      };
    })
    .filter((item) => item.url.trim() !== ""); // Filter out empty lines
  return lines;
}

export { rsplit, formatUrl };
