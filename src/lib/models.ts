import { Duration } from "./duration";

export interface Solve {
  id: number;
  solveTime: Duration;
  timestamp: Date;
}

export enum FaceColor {
  Blue,
  Green,
  Orange,
  Red,
  White,
  Yellow,
}

export enum FaceName {
  Up,
  Down,
  Left,
  Right,
  Front,
  Back,
}

export type CubeFace = Array<Array<FaceColor>>;

export interface Cube {
  faces: Array<CubeFace>;
}
