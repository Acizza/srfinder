The purpose of this program is to automate the tedious process of finding routes in flight simulators that meet an arbitrary amount of conditions. Some conditions include: only taking X amount of time to complete, being in a certain continent, having the arrival be in a different continent and taking less than X hours to reach it, etc.

Usage
=======

The following table contains all of the arguments you can use (note: you **always** have to provide the mach argument):

Name       | Value                  | Description
---------- | ---------------------- | -----------
departure  | ICAO code              | If this argument is present, the ICAO code specified will be used as the departure airport instead of being chosen randomly
mach       | cruising speed         | Affects route times
min        | Time, in hours         | Only display routes that are at least x hours long
max        | Time, in hours         | Only display routes that are x hours long at most
arrivebefore | Hour in 24-hour format | Only display routes that will make you arrive at or before the specified time from now
depcont    | Two-letter continent   | Only display routes that originate from the specified continent
arrcont    | Two-letter continent   | Only display routes that arrive at the specified continent
dest       | ICAO code              | Only display routes that arrive at the specified airport
deptype    | closed, heliport, small, medium, large | Only display departures that are one of the specified airport types
arrtype    | closed, heliport, small, medium, large | Only display arrivals that are one of the specified airport types
sort       | time, name, icao       | Sorts the output of the filtered routes by the specified type
sortorder  | ascending, descending  | Specifies the sorting order routes will be displayed as
autoupdate | true, false            | Specifies whether or not the program should automatically update the airport data
maxroutes  | non-negative number    | Specifies the maximum number of processed routes that will display
numlegs    | non-negative number    | Will find a specified number of legs that meet the other specified conditions

Usage Examples
==============

* To display routes from KSMF that are at least 2 hours long and less than 4 and a half hours long at mach 0.73:
```
./FFilter.exe --mach 0.73 --departure KSMF --min 2:00 --max 4:30
```

* To display routes from KSFO that will make you arrive at a medium airport at or before 5 PM local time when crusing at mach 0.73:
```
./FFilter.exe --mach 0.73 --departure KSFO --arrtype medium --arrivebefore 17:00
```

* To display routes that originate from a random large airport in North America and arrive at a large airport in Europe that take at least 8 hours when crusing at mach 0.73:
```
./FFilter.exe --mach 0.73 --min 8:00 --deptype large --depcont NA --arrtype large --arrcont EU
```

* To display routes that originate from a random medium airport in South America and arrive at a large airport 2 - 4 hours away when cruising at mach 0.73:
```
./FFilter.exe --mach 0.73 --depcont SA --deptype medium --arrtype large --min 2:00 --max 4:00
```

Credits
=======

* [OurAirports](http://ourairports.com) - providing the airports.csv file