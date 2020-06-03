import type CoordPos from "./coord_pos";

export default interface Runway {
  lengthFT?: number;
  widthFT?: number;
  heMarker?: RunwayMarker;
  leMarker?: RunwayMarker;
}

export interface RunwayMarker {
  name: string;
  position: CoordPos;
}
