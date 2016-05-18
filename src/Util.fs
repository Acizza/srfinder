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