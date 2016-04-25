module Airport

open FSharp.Data

type Airports = CsvProvider<"airports.csv">

/// Parses the specified CSV file and returns a map where the key is the ICAO
/// and value is the continent where the ICAO is located
let load (path:string) =
    let csvData = Airports.Load path
    Seq.fold (fun acc (x:Airports.Row) ->
        Map.add x.Ident x.Continent acc
        ) Map.empty csvData.Rows

let exists icao continent list =
    Map.exists (fun k v -> k = icao && v = continent) list