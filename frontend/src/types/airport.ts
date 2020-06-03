import type CoordPos from "./coord_pos";
import type Frequencies from "./frequencies";
import type Runway from "./runway";

export default interface Airport {
  icao: string;
  position: CoordPos;
  runways: Runway[];
  frequencies: Frequencies;
  countryName: string;
}
