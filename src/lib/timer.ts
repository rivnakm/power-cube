import { Duration } from "./duration";

export class Timer {
  private _start: Date | undefined;

  start() {
    this._start = new Date();
  }

  stop(): Duration {
    if (this._start === undefined) {
      throw new Error("cannot start timer that is not running");
    }

    const end = new Date();
    const duration = Duration.fromDates(this._start, end);
    this._start = undefined;

    return duration;
  }

  current(): Duration {
    if (this._start === undefined) {
      return new Duration();
    }

    return Duration.fromDates(this._start, new Date());
  }

  isRunning(): boolean {
    return this._start !== undefined;
  }
}
