The purpose of this program is to automate the tedious process of finding an arrival airport that fulfills an arbitrary amount of conditions. Some conditions include: only taking X amount of time to reach it, being in a certain continent, being in a different continent and taking less than X hours to reach it, etc.

Usage
=====

The basic input layout looks like this:
```
./FFilter.exe <departure ICAO> <filters>
```

The *filters* parameter can include as many filters as you want. The filters you can use are as follows:

Name     | Value                | Description
-------- | -------------------- | -----------
min      | Time, in hours       | Only display routes that are at least x hours long
max      | Time, in hours       | Only display routes that are x hours long at most
arrivebf | Time, in hours       | Only display routes that will make you arrive at or before the specified time from now
oc       | Two-letter continent | Only display routes that originate from the specified continent
dc       | Two-letter continent | Only display routes that arrive at the specified continent
dest     | ICAO code            | Only display routes that arrive at the specified airport

Here are several examples of the filters in action:

* To display routes that are at least 2 hours long and less than 4 and a half hours long:
```
./FFilter.exe <depature> -min 2 -max 4.5
```

* To display routes that will make you arrive at or before 5 PM local time:
```
./FFilter.exe <departure> -arrivebf 17
```

* To display routes that originate in North America and arrive in Europe:
```
./FFilter.exe <departure> -oc NA -dc EU
```