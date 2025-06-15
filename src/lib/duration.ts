export class Duration {
  private _totalMilliseconds: number;

  static fromDates(start: Date, end: Date): Duration {
    const totalMilliseconds = end.getTime() - start.getTime();
    return new Duration(0, 0, 0, totalMilliseconds);
  }

  constructor(hours: number = 0, minutes: number = 0, seconds: number = 0, millis: number = 0) {
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
    return Math.floor(this._totalMilliseconds / 1000);
  }

  get milliseconds() {
    return this._totalMilliseconds % 1000;
  }

  get totalMilliseconds() {
    return this._totalMilliseconds;
  }

  toString(format: string = "c"): string {
    switch (format) {
      case "c":
        return this._toStringConstantFormat();
      case "g":
        return this._toStringGeneralShortFormat();
      case "G":
        return this._toStringGeneralLongFormat();
      default:
        return this._toStringCustomFormat(format);
    }
  }

  private _toStringConstantFormat(): string {
    let str = `${this.hours.toString().padStart(2, "0")}:${this.minutes.toString().padStart(2, "0")}:${this.seconds.toString().padStart(2, "0")}`;

    if (this.milliseconds > 0) {
      str += `.${this.milliseconds.toString().padStart(3, "0")}`;
    }

    return str;
  }

  // TODO: general format should use locale-specific separators
  private _toStringGeneralShortFormat(): string {
    let str = `${this.hours.toString().padStart(1, "0")}:${this.minutes.toString().padStart(2, "0")}:${this.seconds.toString().padStart(2, "0")}`;

    if (this.milliseconds > 0) {
      const millisStr = trimEndChars(this.milliseconds.toString().padStart(3, "0"), "0");
      str += `.${millisStr}`;
    }

    return str;
  }

  private _toStringGeneralLongFormat(): string {
    let str = `${this.hours.toString().padStart(2, "0")}:${this.minutes.toString().padStart(2, "0")}:${this.seconds.toString().padStart(2, "0")}`;

    if (this.milliseconds > 0) {
      str += `.${this.milliseconds.toString().padStart(3, "0")}`;
    }

    return str;
  }

  private _toStringCustomFormat(format: string): string {
    let result = format;
    // TODO: figure out a more clever way to do this
    result = result.replace("hh", this.hours.toString().padStart(2, "0"));
    result = result.replace("h", this.hours.toString().padStart(1, "0"));
    result = result.replace("mm", this.minutes.toString().padStart(2, "0"));
    result = result.replace("m", this.minutes.toString().padStart(1, "0"));
    result = result.replace("ss", this.seconds.toString().padStart(2, "0"));
    result = result.replace("s", this.seconds.toString().padStart(1, "0"));

    result = result.replace("fff", this.milliseconds.toString().padStart(3, "0"));
    result = result.replace(
      "ff",
      Math.floor(this.milliseconds / 10)
        .toString()
        .padStart(2, "0"),
    );
    result = result.replace(
      "f",
      Math.floor(this.milliseconds / 100)
        .toString()
        .padStart(1, "0"),
    );

    return result;
  }
}

function trimEndChars(str: string, char: string): string {
  for (let i = str.length - 1; i >= 0; i--) {
    if (str.charAt(i) != char) {
      return str.slice(0, i + 1);
    }
  }

  return "";
}
