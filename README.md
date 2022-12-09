# dbdepartureinfo
A program, that displays departures for multiple train stations in germany.
To get this information the dbf api is used: "https://dbf.finalrewind.org/"

![alt text](https://github.com/AndreasMuehlmann/dbdepartureinfo/blob/main/example.png)

Author: Andreas Mühlmann

GitHub repository: "https://github.com/AndreasMuehlmann/dbdepartureinfo"

## Quickstart
Clone the git repository with
"git clone https://github.com/AndreasMuehlmann/dbdepartureinfo.git".

If git is not installed download the
repository from the website via the zip-archive (click on the green button).

## Usage
Run the executable on Linux with ".\target\release\fraktalgen".

## Playing with the source code
The project is written in rust so you need to install the rust compiler.
To do that visit this website: "https://www.rust-lang.org/learn/get-started"
If that is done run "cargo run", to compile and run the program.

Additionally the druid gui framework is used to make the view: "https://github.com/linebender/druid".
