The purpose of this program is to automate the tedious process of finding routes in flight simulators that meet an arbitrary amount of conditions. Some conditions include: only taking X amount of time to complete, being in a certain continent, having the arrival be in a different continent and taking less than X hours to reach it, etc.

Usage
=====

```
./FFilter.exe <mode> <mode parameters> <filters>
```

The mode argument specifies whether or not you want to provide a departure airport yourself, or have the program find one for you.

If you already know the airport you want to depart from, use **depart** as the mode.
If you want the program to find a departure airport for you, use **random** as the mode.

There will be different parameters that each mode requires you to provide.
For **depart**, you will need to provide the departure ICAO and speed in mach that you plan to cruise at.
For **random**, you will only need to provide the cruising speed in mach.

(note: the typical cruise speed for a commercial jet is around mach 0.73 - 0.83)

After providing the necessary parameters for the chosen mode, you can then proceed to specify filters, as described further below, that you'd like to apply to potential arrival airports (and the potential departure airport, if you're using the random mode.)

To see everything in action, jump to the [Usage Examples](#usage-examples) section.

Filters
=======
You can specify as many filters as you want and they can be combined in any order. The filters you can use are as follows:

Name     | Value                  | Description
-------- | ---------------------- | -----------
min      | Time, in hours         | Only display routes that are at least x hours long
max      | Time, in hours         | Only display routes that are x hours long at most
arrivebf | Hour in 24-hour format | Only display routes that will make you arrive at or before the specified time from now
depcont  | Two-letter continent   | Only display routes that originate from the specified continent
arrcont  | Two-letter continent   | Only display routes that arrive at the specified continent
dest     | ICAO code              | Only display routes that arrive at the specified airport
deptype  | closed, heliport, small, medium, large | Only display departures that are one of the specified airport types
arrtype  | closed, heliport, small, medium, large | Only display arrivals that are one of the specified airport types
sort     | time, name, icao       | Sorts the output of the filtered routes by the specified type in descending order

Usage Examples
==============

* To display routes from KSMF that are at least 2 hours long and less than 4 and a half hours long at mach 0.73:
```
./FFilter.exe depart KSMF 0.73 -min 2 -max 4.5
```

* To display routes from KSFO that will make you arrive at a medium airport at or before 5 PM local time when crusing at mach 0.73:
```
./FFilter.exe depart KSFO 0.73 -arrtype medium -arrivebf 17:00
```

* To display routes that originate from a random large airport in North America and arrive at a large airport in Europe that take at least 8 hours when crusing at mach 0.73:
```
./FFilter.exe random 0.73 -min 8:00 -deptype large -depcont NA -arrtype large -arrcont EU
```

* To display routes that originate from a random medium airport in South America and arrive at a large airport 2 - 4 hours away when cruising at mach 0.73:
```
./FFilter.exe random 0.73 -depcont SA -deptype medium -arrtype large -min 2:00 -max 4:00
```

Credits
=======

* [OurAirports](http://ourairports.com) - providing the airports.csv file