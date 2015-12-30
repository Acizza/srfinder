Usage
=====

At the moment, you need to provide the ID for the VA you want routes from. To retrieve the ID, go [here](http://fscloud-infotool.de/index.php?page=vasystem&subpage=vrank) and select your airline. Then, in the address bar, copy the set of numbers at the end of the URL.

With the VA ID in hand, you can launch the program like this:
```
./FFilter.exe <VA ID> <filters>
```

The *filters* parameter at the end can include as many filters as you want. The filters you can use are as follows:

Name     | Value                | Description
-------- | -------------------- | -----------
min      | Time, in hours       | Only display routes that are at least x hours long
max      | Time, in hours       | Only display routes that are x hours long at most
arrivebf | Time, in hours       | Only display routes that will make you arrive at or before the specified time from now
oc       | Two-letter continent | Only display routes that originate from the specified continent
dc       | Two-letter continent | Only display routes that arrive at the specified continent
origin   | ICAO code            | Only display routes that originate from the specified airport
dest     | ICAO code            | Only display routes that arrive at the specified airport

Here are several examples of the filters in action:

* To display routes that are at least 2 hours long and less than 4 and a half hours long:
```
./FFilter.exe -min 2 -max 4.5
```

* To display routes that will make you arrive at or before 5 PM local time:
```
./FFilter.exe -arrivebf 17
```

* To display routes that originate in North America and arrive in Europe:
```
./FFilter.exe -oc NA -dc EU
```

* To display routes that originate from Sacramento Intl. and are at least 2 hours long:
```
./FFilter.exe -origin KSMF -min 2
```

Note that you can combine as many filters as you want.
