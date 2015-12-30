module Main

open FSharp.Collections.ParallelSeq
open System
open Route
open Option

/// Convert a decimal containing hours to it's hour:minute representation
let decToTime hours =
    let minutes = (hours % 1.) * 60.
    sprintf "%d:%02d"
        (floor hours |> int)
        (floor minutes |> int)

let processRoutes vaID options =
    let airports = Airport.load "airports.csv"
    let routes =
        let filter = function
            | {Route.time = t},   MinTime m         -> t >= m
            | {Route.time = t},   MaxTime m         -> t <= m
            | {Route.time = t},   ArriveBefore lt   -> DateTime.Now.TimeOfDay + t <= lt
            | {Route.origin = o}, OriginContinent c -> Airport.exists o c airports
            | {Route.dest = d},   DestContinent c   -> Airport.exists d c airports
            | {Route.origin = o}, OriginAirport a   -> o = a
            | {Route.dest = d},   DestAirport a     -> d = a

        Route.parseAll vaID
        |> PSeq.filter (fun r -> Seq.forall (fun o -> filter (r, o)) options)
        |> PSeq.sortBy (fun r -> r.time.TotalMinutes)

    for r in routes do
        printfn "Origin: %s\nDest: %s\nTime: %s\nRoute: %s\n"
            r.origin
            r.dest
            (decToTime <| r.time.TotalHours)
            r.route

[<EntryPoint>]
let main args =
    match args |> Array.toList with
    | [] -> Console.WriteLine "Usage: <VA id> <options>"
    | vaID::xs ->
        let options = Option.readAll xs
        match options with
        | [] -> Console.WriteLine "No options specified"
        | _ -> processRoutes vaID options
    0
