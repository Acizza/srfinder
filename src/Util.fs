module Util

open System

module Convert =
    module Meter =
        let toNauticalMiles = (*) 0.000539957

module Type =
    let tryParse f x =
        match f x with
        | (true, v)  -> Some v
        | (false, _) -> None

    let (|Double|_|) = tryParse Double.TryParse

module Coord =
    type T = {
        Lat : double
        Lon : double
    }

    let create lat lon = {
        Lat = lat
        Lon = lon
    }

    /// Returns the distance between two coordinates in meters
    let distance c1 c2 =
        // Haversine formula
        let toRads deg = deg * Math.PI / 180.
        let r = 6371000. // Earth's radius in meters

        let lat1 = toRads c1.Lat
        let lat2 = toRads c2.Lat
        let lat  = lat2 - lat1
        let lon  = toRads (c2.Lon - c1.Lon)

        let a = sin (lat / 2.) ** 2. + cos lat1 * cos lat2 * sin (lon / 2.) ** 2.
        let c = 2. * atan2 (sqrt a) (sqrt (1. - a))
        r * c

module Option =
    /// Flipped version of defaultArg
    let defaultArg x y = defaultArg y x

/// Convert a decimal containing hours to its hour:minute representation
let formatTime hours =
    let minutes = (hours % 1.) * 60.
    sprintf "%d:%02d"
        (floor hours |> int)
        (floor minutes |> int)