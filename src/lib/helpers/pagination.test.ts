import { computeCurrentGroup, computePageGroups, createPageList, GROUP_OFFSET, MAX_FULL_PAGES } from './pagination';

describe('createPageList', () => {
  test('creates array from number of pages', () => {
    const pages = 16;
    const pageList = createPageList(pages);

    expect(pageList).toHaveLength(pages);

    for (let i = 0; i < pages; i++) {
      expect(pageList[i]).toEqual(i + 1);
    }
  });

  test('creates empty array when pages is zero', () => {
    const pages = 0;
    const pageList = createPageList(pages);

    expect(pageList).toHaveLength(0);
  })
})

describe('computeCurrentGroup', () => {
  //pages <= 10 -> 1 2 3 4 5 6...
  describe('shrink is not needed', () => {
    const pages = MAX_FULL_PAGES;

    test('1st page', () => {
      const currentPage = 1;
      const group = computeCurrentGroup(currentPage, pages)

      const expectedLength = 10;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + 1);
      }
    });

    test('5th page', () => {
      const currentPage = 5;
      const group = computeCurrentGroup(currentPage, pages)

      const expectedLength = 10;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + 1);
      }
    });

    test('10th page', () => {
      const currentPage = 10;
      const group = computeCurrentGroup(currentPage, pages)

      const expectedLength = 10;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + 1);
      }
    });
  })

  describe('need shrink', () => {
    const pages = 30;
    const offset = GROUP_OFFSET;

    //current 1 -> 1 2 3 ... l
    test('1st page', () => {
      const currentPage = 1;
      const group = computeCurrentGroup(currentPage, pages);

      const expectedLength = 3;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + 1);
      }
    })

    //current 2 -> 1 2 3 4 ... l
    test('2nd page', () => {
      const currentPage = 2;
      const group = computeCurrentGroup(currentPage, pages);

      const expectedLength = 4;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + 1);
      }
    })

    //current 3 -> 1 2 3 4 5 ... l
    test('3rd page', () => {
      const currentPage = 3;
      const group = computeCurrentGroup(currentPage, pages);

      const expectedLength = 5;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + 1);
      }
    })

    //current 4 -> 1 2 3 4 5 6 ... l
    test('4th page', () => {
      const currentPage = 4;
      const group = computeCurrentGroup(currentPage, pages);

      const expectedLength = 6;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + 1);
      }
    })

    //current 5 -> 1 ... 3 4 5 6 7 ... l
    test('5th page', () => {
      const currentPage = 5;
      const group = computeCurrentGroup(currentPage, pages);

      const expectedLength = 5;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + currentPage - offset);
      }
    })

    //current l-4 -> 1 ... l-6 l-5 l-4 l-3 l-2 ... l
    test('last - 4 page', () => {
      const currentPage = pages - 4;
      const group = computeCurrentGroup(currentPage, pages);

      const expectedLength = 5;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + currentPage - offset);
      }
    })

    //current l-3 -> 1 ... l-5 l-4 l-3 l-2 l-1 l
    test('last - 3 page', () => {
      const currentPage = pages - 3;
      const group = computeCurrentGroup(currentPage, pages);

      const expectedLength = 6;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + currentPage - offset);
      }
    })

    //current l-2 -> 1 ... l-4 l-3 l-2 l-1 l
    test('last - 2 page', () => {
      const currentPage = pages - 2;
      const group = computeCurrentGroup(currentPage, pages);

      const expectedLength = 5;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + currentPage - offset);
      }
    })

    //current l-1 -> 1 .. l-3 l-2 l-1 l
    test('last - 1 page', () => {
      const currentPage = pages - 1;
      const group = computeCurrentGroup(currentPage, pages);

      const expectedLength = 4;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + currentPage - offset);
      }
    })

    //current l -> 1 ... l-2 l-1 l
    test('last page', () => {
      const currentPage = pages;
      const group = computeCurrentGroup(currentPage, pages);

      const expectedLength = 3;
      expect(group).toHaveLength(expectedLength);

      for (let i = 0; i < expectedLength; i++) {
        expect(group[i]).toEqual(i + currentPage - offset);
      }
    })
  })
})

describe('computePageGroups', () => {
  test('without shrink', () => {
    const pages = 8;
    const { start, end } = computePageGroups(7, pages);

    expect(start).toBeNull();
    expect(end).toBeNull();
  });

  test('only end', () => {
    const pages = 20;
    const { start, end } = computePageGroups(3, pages);

    expect(start).toBeNull();
    expect(end).toEqual(pages);
  });

  test('only start', () => {
    const pages = 20;
    const { start, end } = computePageGroups(19, pages);

    expect(start).toEqual(1);
    expect(end).toBeNull();
  });

  test('has start and end', () => {
    const pages = 20;
    const { start, end } = computePageGroups(15, pages);

    expect(start).toEqual(1);
    expect(end).toEqual(pages);
  });
})