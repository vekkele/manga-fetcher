export const MAX_FULL_PAGES = 10;
export const GROUP_OFFSET = 2;

export function createPageList(pages: number) {
  return Array(pages)
    .fill(0)
    .map((_, i) => i + 1);
}

export function computeCurrentGroup(currentPage: number, pages: number) {
  if (pages <= MAX_FULL_PAGES) return createPageList(pages);

  let start = Math.max(1, currentPage - GROUP_OFFSET);
  let end = Math.min(pages, currentPage + GROUP_OFFSET);

  if (start - 1 === 1) {
    start = 1;
  }

  if (end + 1 === pages) {
    end = pages;
  }

  return Array(end - start + 1).fill(0).map((_, i) => start + i);
}

export function computePageGroups(currentPage: number, pages: number) {
  const currentGroup = computeCurrentGroup(currentPage, pages);

  const start = currentGroup[0] === 1 ? null : 1;
  const end = currentGroup[currentGroup.length - 1] === pages ? null : pages;

  return {
    start,
    currentGroup,
    end,
  }
}