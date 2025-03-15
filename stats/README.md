# stats

<img width='200' height='200' src="./docs/public/logo.svg">

> [!NOTE]
> ShortLink stats-server

### Goal

The functional goal of a microservice for saving stats would be to provide a reliable
and efficient way to store and manage data about the usage and performance of a
larger system. This would involve several key tasks, including:

1. Collecting data from various sources within the system, such as logs, metrics,
   or other data sources.
2. Storing the data in a secure and organized manner, using a database or other
   data storage solution.
3. Providing access to the data through APIs or other interfaces, allowing other
   services or applications to retrieve and use the data as needed.
4. Ensuring that the data is accurate and up-to-date, by regularly updating and
   maintaining the data storage solution.
5. Providing tools or features for analyzing and visualizing the data, allowing
   users to easily understand and interpret the data and make informed decisions based on it.

Overall, the functional goal of a microservice for saving stats would be to provide
a reliable and flexible way to manage and use data about the performance of a larger
system, enabling users to make data-driven decisions and improve the system's
performance over time.

### Getting started

We use Makefile for build and deploy.

```bash
$> make help # show help message with all commands and targets
```

### Stack

- [C++](https://isocpp.org/)
- DataBase
    - TimeSeries
- Tooling
    - [CMake](https://cmake.org/)
    - [Bazel](https://bazel.build/)
    - [Conan](https://conan.io/)
- Observability
    - [Promehteus](https://prometheus.io/)
