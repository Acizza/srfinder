The purpose of this program is to automate the tedious process of finding an arrival airport in flight simulators that meet an arbitrary amount of conditions. Some conditions include: only taking X amount of time to reach it, being in a certain continent, being in a different continent and taking less than X hours to reach it, etc.

Usage
=====

The basic input layout looks like this:
```
./FFilter.exe <departure ICAO> <cruise speed in mach> <filters>
```
(note: the typical cruise speed for a commercial jet is around mach 0.73 - 0.83)

The *filters* parameter can include as many filters as you want and can be combined in any order. The filters you can use are as follows:

Name     | Value                | Description
-------- | -------------------- | -----------
min      | Time, in hours       | Only display routes that are at least x hours long
max      | Time, in hours       | Only display routes that are x hours long at most
arrivebf | Time, in hours       | Only display routes that will make you arrive at or before the specified time from now
dc       | Two-letter continent | Only display routes that originate from the specified continent
ac       | Two-letter continent | Only display routes that arrive at the specified continent
dest     | ICAO code            | Only display routes that arrive at the specified airport
type     | closed, heliport, small, medium, large | Only display routes that are one of the specified airport types
sort     | time, name, icao     | Sorts the output of the filtered routes by the specified type in descending order

Here are several examples of the filters in action:

* To display routes that are at least 2 hours long and less than 4 and a half hours long:
```
./FFilter.exe <depature> <mach> -min 2 -max 4.5
```

* To display routes that will make you arrive at or before 5 PM local time:
```
./FFilter.exe <departure> <mach> -arrivebf 17
```

* To display routes that originate in North America and arrive in Europe:
```
./FFilter.exe <departure> <mach> -dc NA -ac EU
```

Credits
=======

* [OurAirports](http://ourairports.com) - providing the airports.csv file