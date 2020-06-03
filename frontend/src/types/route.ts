import type Airport from "./airport";
import type Time from "./time";

export default interface Route {
  from: Airport;
  to: Airport;
  distance: number;
  time: Time;
}
