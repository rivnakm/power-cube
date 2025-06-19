import { expect, test } from "vitest";
import { Duration } from "./duration";

test("milliseconds getters", () => {
  const duration = new Duration(0, 0, 1, 100);
  expect(duration.milliseconds).toBe(100);
  expect(duration.totalMilliseconds).toBe(1100);
});

test("seconds getters", () => {
  const duration = new Duration(0, 1, 10, 500);
  expect(duration.seconds).toBe(10);
  expect(duration.totalSeconds).toBe(70);
});

test("minutes getters", () => {
  const duration = new Duration(1, 10, 30, 500);
  expect(duration.minutes).toBe(10);
  expect(duration.totalMinutes).toBe(70);
});

test("hours getters", () => {
  const duration = new Duration(70, 10, 30, 500);
  expect(duration.hours).toBe(10);
  expect(duration.totalHours).toBe(70);
});

test("seconds getters with milliseconds overflow", () => {
  const duration = new Duration(0, 0, 2, 2500);
  expect(duration.seconds).toBe(4);
  expect(duration.totalSeconds).toBe(4);
});

test("minutes getters with seconds overflow", () => {
  const duration = new Duration(0, 2, 140, 0);
  expect(duration.minutes).toBe(4);
  expect(duration.totalMinutes).toBe(4);
});

test("hours getters with minutes overflow", () => {
  const duration = new Duration(2, 140, 0, 0);
  expect(duration.hours).toBe(4);
  expect(duration.totalHours).toBe(4);
});

test("fromDates", () => {
  const start = new Date(2025, 0, 1, 0, 0, 0);
  const end = new Date(2025, 0, 1, 1, 2, 3);

  const duration = Duration.fromDates(start, end);

  expect(duration.hours).toBe(1);
  expect(duration.minutes).toBe(2);
  expect(duration.seconds).toBe(3);
});

const formatTestCases = [
  {
    format: "c",
    duration: new Duration(),
    expected: "00:00:00",
  },
  {
    format: "c",
    duration: new Duration(1, 2, 3, 4),
    expected: "01:02:03.004",
  },
  {
    format: "g",
    duration: new Duration(),
    expected: "0:00:00",
  },
  {
    format: "g",
    duration: new Duration(1, 2, 3, 4),
    expected: "1:02:03.004",
  },
  {
    format: "g",
    duration: new Duration(12, 2, 3, 400),
    expected: "12:02:03.4",
  },
  {
    format: "G",
    duration: new Duration(),
    expected: "00:00:00",
  },
  {
    format: "G",
    duration: new Duration(1, 2, 3, 4),
    expected: "01:02:03.004",
  },
  {
    format: "h",
    duration: new Duration(1, 2, 3, 4),
    expected: "1",
  },
  {
    format: "hh",
    duration: new Duration(1, 2, 3, 4),
    expected: "01",
  },
  {
    format: "h:m",
    duration: new Duration(1, 2, 3, 4),
    expected: "1:2",
  },
  {
    format: "hh:mm",
    duration: new Duration(1, 2, 3, 4),
    expected: "01:02",
  },
  {
    format: "h:m:s",
    duration: new Duration(1, 2, 3, 4),
    expected: "1:2:3",
  },
  {
    format: "hh:mm:ss",
    duration: new Duration(1, 2, 3, 4),
    expected: "01:02:03",
  },
  {
    format: "h:m:s.f",
    duration: new Duration(1, 2, 3, 421),
    expected: "1:2:3.4",
  },
  {
    format: "hh:mm:ss.ff",
    duration: new Duration(1, 2, 3, 421),
    expected: "01:02:03.42",
  },
  {
    format: "hh:mm:ss.fff",
    duration: new Duration(1, 2, 3, 421),
    expected: "01:02:03.421",
  },
];

for (let { format, duration, expected } of formatTestCases) {
  test(`format; ${format}; ${expected}`, () => {
    expect(duration.toString(format)).toBe(expected);
  });
}
