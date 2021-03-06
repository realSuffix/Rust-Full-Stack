[How to use React with Rust Actix]: https://www.steadylearner.com/blog/read/How-to-use-React-with-Rust-Actix
[How to use Docker commands]: https://www.steadylearner.com/blog/read/How-to-use-Docker-commands
[How to use Docker with Rust]: https://www.steadylearner.com/blog/read/How-to-use-Docker-with-Rust

# React Full Stack Example

React and various servers to test it. It will be improved with Blog Posts at Steadylearner.

![user-signup](/src/images/screenshot/user-signup.png)

## How to test frontend

React, Webpack, Formik, Jest, React-Testing-Library, Cypress etc.

```console
$nvm install 12.3.1
$nvm use 12.3.1
$yarn
$yarn start
```

## How to test it with server

You can compare the performance with **$./loadtest.bash**.

I would use Express normally or Restify for simple Rest JavaScript project.

If I care for speed, the choice will be among Actix, Golang, Vibora. Actix shows the performance as expected. Golang was way better than I thought. The result from Vibora was similar to the others. I think it is underated.

Rocket, Django don't seem to be adequate for many concurrent requests.

### Express

Webpack Dev server is based on Express. Therefore, prototype with it first. We will use it to learn how to deploy these web server to AWS. Only ports and frameworks will be different.

```console
./express.bash
```

### Warp

```console
./warp.bash
```

The result from loadtest. Some errors but reasonable speed.

```console
INFO Requests: 1086, requests per second: 216, mean latency: 133.9 ms
INFO Requests: 2548, requests per second: 295, mean latency: 582.9 ms
INFO Requests: 4011, requests per second: 293, mean latency: 1354.5 ms
INFO Errors: 115, accumulated errors: 115, 2.9% of total requests
INFO
INFO Target URL:          http://0.0.0.0:8000/
INFO Max time (s):        20
INFO Concurrency level:   10
INFO Agent:               keepalive
INFO Requests per second: 2000
INFO
INFO Completed requests:  5589
INFO Total errors:        492
INFO Total time:          20.006069541 s
INFO Requests per second: 279
INFO Mean latency:        1037.8 ms
INFO
INFO Percentage of the requests served within a certain time
INFO   50%      747 ms
INFO   90%      2419 ms
INFO   95%      2872 ms
INFO   99%      4934 ms
INFO  100%      7192 ms (longest request)
INFO
INFO  100%      7192 ms (longest request)
INFO
INFO   404:   492 errors
```

### Rocket

Use it to test you can deploy React Rocket application to AWS. Then, you should separate the project. Use **/static/<file..>** instead of current **/<file..>** in **routes/static_files.rs** to serve static files. It will help the Rocket server not to show errors from it.

```console
./rocket.bash
```

### Actix

[How to use React with Rust Actix]

```console
./actix.bash
```

The result from loadtest. No errors and very fast.

```console
INFO Requests: 0, requests per second: 0, mean latency: 0 ms
INFO Requests: 1797, requests per second: 360, mean latency: 43.2 ms
INFO Requests: 4855, requests per second: 612, mean latency: 34.4 ms
INFO Requests: 9126, requests per second: 833, mean latency: 29.6 ms
INFO
INFO Target URL:          http://0.0.0.0:8000/
INFO Max time (s):        20
INFO Concurrency level:   10
INFO Agent:               keepalive
INFO Requests per second: 2000
INFO
INFO Completed requests:  13300
INFO Total errors:        0
INFO Total time:          20.005901511 s
INFO Requests per second: 665
INFO Mean latency:        32.4 ms
INFO
INFO Percentage of the requests served within a certain time
INFO   50%      26 ms
INFO   90%      59 ms
INFO   95%      64 ms
INFO   99%      75 ms
INFO  100%      110 ms (longest request)
```

### Restify

```console
./restify.bash
```

### Golang

```console
./golang.bash
```

The reults from loadtest. It is very fast and no errors. It is made for this.

```console
INFO Requests: 0, requests per second: 0, mean latency: 0 ms
INFO Requests: 3820, requests per second: 772, mean latency: 9 ms
INFO Requests: 11529, requests per second: 1541, mean latency: 5 ms
INFO Requests: 20169, requests per second: 1729, mean latency: 4.0 ms
INFO 
INFO Target URL:          http://0.0.0.0:8000/
INFO Max time (s):        20
INFO Concurrency level:   10
INFO Agent:               keepalive
INFO Requests per second: 2000
INFO 
INFO Completed requests:  27881
INFO Total errors:        0
INFO Total time:          20.002745433 s
INFO Requests per second: 1394
INFO Mean latency:        5.4 ms
INFO 
INFO Percentage of the requests served within a certain time
INFO   50%      4 ms
INFO   90%      8 ms
INFO   95%      10 ms
INFO   99%      19 ms
INFO  100%      339 ms (longest request)
```


### Vibora

You should use **$python3.6 -m venv python** and include Vibora folder in it.

```console
./vibora.bash
```

### Django

Some Python frameworks requires you to include all the static files in /static folder and not /. So it was not easy fo make it work.

This will be the last framework to compare for a while.

You should use **$python -m venv python** and include Django folder in it.

```console
./django.bash
```

## Blog posts

1. [How to use Webpack with React](https://www.steadylearner.com/blog/read/How-to-use-Webpack-with-React)
2. [How to use Cypress with React](https://www.steadylearner.com/blog/read/How-to-use-Cypress-with-React)
3. [How to use gRPC with Rust Tonic and Postgresql database](https://www.steadylearner.com/blog/read/How-to-use-gRPC-with-Rust-Tonic-and-Postgresql-database)
4. [How to use Docker commands]
5. [How to use Docker with Rust]

## Screenshots

![user-result](/src/images/screenshot/user-result.png)
![user-list](/src/images/screenshot/user-list.png)
