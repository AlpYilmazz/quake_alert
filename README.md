# Earthquake Alert #

Application uses the website of [Kandilli Rasathanesi]
to scrape information about the earthquakes happening.

The application runs indefinitely and:
 - Cheks the website for new earthquakes every 10 seconds
 - Filters them according to given alert rule
 - Emails an alert mail with information about the earthquakes

The job of this application is only to send emails about
the earthquakes significant to your location,
alerting with sound notifications is handled by another
component.

### Mobile Alert Notification ###

Actual alerts notifications are handled by an android application
called [eNotify Lite] for my case, one can use other similar
applications. [eNotify Lite] scans incoming mails and
matches them against conditions to initiate an alert.
I have setup a condition to look for "YAKIN DEPREM" in the
subject field of incoming mails.

## Configuration ##

The executable requires an `config.ron` file in the working directory
to configure things such as email credentials and alert rule.

An example config can be found in the `config.example.ron` file:
```
ProgramConfig (
    run_mode: Debug,
    account: AlertAccount (
        mail: "mail_address",
        password: "mail_password",
    ),
    rule: AlertRule (
        origin: Coord ( // Istanbul, Sisli
            latitude: 41.0780,
            longitude: 29.0040,
        ),
        search_radius_km_1: 300.0,
        min_magnitude_1: 0.0,
        search_radius_km_2: 400.0,
        min_magnitude_2: 4.0,
    ),
)
```
### Alert Rule ###

Alert rule defines a search circle around the given `origin`
and filters the earthquakes such that they lie inside the circle
and have magnitudes greater than some `min_magnitude` value.
Min magnitude to filter with is calculated from two radii
`search_radius_km_1` and `search_radius_km_2`.

 - Anything closer than `search_radius_km_1` has `min_magnitude`
 as `min_magnitude_1`
 - Anything farther than `search_radius_km_2` is automatically
 filtered out
 - Anything between the two, the `min_magnitude` is calculated by
 linearly interpolating between the two min magnitude values.

The surface distance between two coordinates is calculated using the
[Haversine Formula].



[eNotify Lite]: https://play.google.com/store/apps/details?id=com.hermes.enotifylite&hl=tr&gl=US&pli=1
[Kandilli Rasathanesi]: http://www.koeri.boun.edu.tr/scripts/sondepremler.asp
[Haversine Formula]: https://en.wikipedia.org/wiki/Haversine_formula

