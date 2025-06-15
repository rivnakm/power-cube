import { expect, test } from "vitest";
import { Duration } from "./duration";

test("milliseconds getters", () => {
  const duration = new Duration(0, 0, 1, 100);
  expect(duration.milliseconds).toBe(100);
  expect(duration.totalMilliseconds).toBe(1100);
})

test("seconds getters", () => {
  const duration = new Duration(0, 1, 10, 500);
  expect(duration.seconds).toBe(10);
  expect(duration.totalSeconds).toBe(70);
})

test("minutes getters", () => {
  const duration = new Duration(1, 10, 30, 500);
  expect(duration.minutes).toBe(10);
  expect(duration.totalMinutes).toBe(70);
})

test("hours getters", () => {
  const duration = new Duration(70, 10, 30, 500);
  expect(duration.hours).toBe(10);
  expect(duration.totalHours).toBe(70);
})

test("seconds getters with milliseconds overflow", () => {
  const duration = new Duration(0, 0, 2, 2500);
  expect(duration.seconds).toBe(4);
  expect(duration.totalSeconds).toBe(4);
})

test("minutes getters with seconds overflow", () => {
  const duration = new Duration(0, 2, 140, 0);
  expect(duration.minutes).toBe(4);
  expect(duration.totalMinutes).toBe(4);
})

test("hours getters with minutes overflow", () => {
  const duration = new Duration(2, 140, 0, 0);
  expect(duration.hours).toBe(4);
  expect(duration.totalHours).toBe(4);
})
