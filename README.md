srfinder
========

This is a program to find (direct) flight routes for flight simulators via a range of filters. Filter types include airport type, country, runway length, and minimum / maximum time between routes.

The application uses a web interface to plot the selected route on an interative map, display airport runways on the map, and to provide various runway / frequency details.

Usage
=====

As the application uses a web interface, all you need to do is run the program and it will open automatically by default. If you don't want the web interface to open automatically, you can launch the program with the **-m** flag.

The filter options should be mostly self-explanatory. However, it is worth noting that the runway length filter can take inputs in one of the several following forms:

Value | Form
----- | ----
>9500 | Greater than or equal to
=9500 | Equal to
<9500 | Less than or equal to
9500 12000 | A range

Credits
=======

[OurAirports](http://ourairports.com/) - Provides worldwide airport data that makes applications like this viable.