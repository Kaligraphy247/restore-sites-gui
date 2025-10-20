import type { SiteEntry } from "$lib/types/models";

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

function formatUrl(sites: string): SiteEntry[] {
  let lines = sites
    .trimStart()
    .trimEnd()
    .split("\n")
    ?.map((item): SiteEntry => {
      const trimmedItem = item.trim();
      if (!trimmedItem) return { title: "", url: "" };

      // Use regex to find URLs (http:// or https://)
      // handle query parameters with special chars
      const urlMatch = trimmedItem.match(/(https?:\/\/[^\s"']+)/);

      if (urlMatch) {
        const url = urlMatch[1];
        const urlIndex = trimmedItem.indexOf(url);
        let title = trimmedItem.substring(0, urlIndex).trim();

        // If title is empty, use the URL as title
        if (!title) {
          title = url;
        }

        return {
          title: title,
          url: url,
        };
      } else {
        // If no URL pattern found, try splitting by last space
        const lastSpaceIndex = trimmedItem.lastIndexOf(" ");
        if (lastSpaceIndex > 0) {
          const title = trimmedItem.substring(0, lastSpaceIndex).trim();
          const url = trimmedItem.substring(lastSpaceIndex + 1).trim();

          // Simple URL validation - must contain a dot or start with http
          if (url.includes(".") || url.startsWith("http")) {
            return {
              title: title || url,
              url: url.startsWith("http") ? url : `https://${url}`,
            };
          }
        }

        // Fallback: treat entire line as URL if it looks like one
        if (trimmedItem.includes(".") || trimmedItem.startsWith("http")) {
          const url = trimmedItem.startsWith("http")
            ? trimmedItem
            : `https://${trimmedItem}`;
          return {
            title: url,
            url: url,
          };
        }

        return { title: "", url: "" };
      }
    })
    .filter((item) => item.url.trim() !== ""); // Filter out empty URLs
  return lines;
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diffInMs = now.getTime() - date.getTime();
  const diffInHours = diffInMs / (1000 * 60 * 60);

  if (diffInHours < 1) {
    const diffInMins = Math.floor(diffInMs / (1000 * 60));
    return `${diffInMins}m ago`;
  } else if (diffInHours < 24) {
    return `${Math.floor(diffInHours)}h ago`;
  } else {
    const diffInDays = Math.floor(diffInHours / 24);
    return `${diffInDays}d ago`;
  }
}

/**
 * Click outside action for Svelte components
 * Triggers callback when user clicks outside the element
 */
function clickOutside(node: HTMLElement, callback: () => void) {
  const handleClick = (event: MouseEvent) => {
    if (!node.contains(event.target as Node)) {
      callback();
    }
  };

  document.addEventListener("click", handleClick, true);

  return {
    destroy() {
      document.removeEventListener("click", handleClick, true);
    },
  };
}

export { rsplit, formatUrl, formatDate, clickOutside };
