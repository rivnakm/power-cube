export class Duration {
  private _totalMilliseconds: number;

  static fromDates(start: Date, end: Date): Duration {
    const totalMilliseconds = end.getTime() - start.getTime()
    return new Duration(0, 0, 0, totalMilliseconds);
  }

  constructor(hours: number, minutes: number, seconds: number, millis: number) {
    minutes += hours * 60;
    seconds += minutes * 60;
    millis += seconds * 1000;

    this._totalMilliseconds = millis;
  }

  get hours() {
    return Math.floor(this._totalMilliseconds / 1000 / 60 / 60) % 60;
  }

  get totalHours() {
    return Math.floor(this._totalMilliseconds / 1000 / 60 / 60);
  }

  get minutes() {
    return Math.floor(this._totalMilliseconds / 1000 / 60) % 60;
  }

  get totalMinutes() {
    return Math.floor(this._totalMilliseconds / 1000 / 60);
  }

  get seconds() {
    return Math.floor(this._totalMilliseconds / 1000) % 60;
  }

  get totalSeconds() {
    return Math.floor(this._totalMilliseconds / 1000)
  }

  get milliseconds() {
    return this._totalMilliseconds % 1000;
  }

  get totalMilliseconds() {
    return this._totalMilliseconds;
  }
}
