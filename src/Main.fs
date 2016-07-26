module Main

open Airport
open Args
open Argu
open Route
open System
open Util
open Util.Type

type ProcessMode =
    | Leg     of maxRoutes:int option
    | Segment of numLegs:int

let dispatchMode routeInfo depICAO airports mode =
    let departure =
        depICAO
        |> Option.bind (fun icao -> Airport.findFromICAO icao airports)
        |> Option.bindNone (Route.randomDeparture routeInfo.Filters airports)

    match departure with
    | Some dep ->
        match mode with
        | Leg maxRoutes ->
            Route.Leg.filterAndDisplay maxRoutes routeInfo dep airports
            Ok ()
        | Segment numLegs ->
            Route.Segment.findAndDisplay numLegs routeInfo dep airports
    | None -> Error "No departure airport found"

let processAirports args =
    let filters = Args.parseFilters args

    let airports =
        Airport.loadAll DataFile.path
        |> Seq.toArray

    let routeInfo = {
        Mach     = Args.getValue <@ Mach @> args
        Filters  = filters
        SortType =
            args
            |> Args.tryGetValue <@ Sort @>
            |> Option.defaultArg Time
        SortOrder =
            args
            |> Args.tryGetValue <@ SortOrder @>
            |> Option.defaultArg Descending
    }

    let mode =
        match Args.tryGetValue <@ NumLegs @> args with
        | Some num -> Segment num
        | None     -> Leg (Args.tryGetValue <@ MaxRoutes @> args)

    let departure = Args.tryGetValue <@ Departure @> args
    let result    = dispatchMode routeInfo departure airports mode
    
    match result with
    | Ok _      -> ()
    | Error msg -> eprintfn "Failure during route processing: %s" msg

[<EntryPoint>]
let main args =
    match Args.readAll args with
    | Some results ->
        let autoUpdateEnabled =
            Args.tryGetValue (<@ AutoUpdate @>) results
            |> Option.defaultArg true

        if autoUpdateEnabled then
            match DataFile.verifyAndUpdate () with
            | Ok _      -> ()
            | Error msg -> eprintfn "Error updating airport data: %s" msg

        processAirports results
    | None -> ()

    0