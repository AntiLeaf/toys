#pragma once

#include <iostream>
#include <type_traits>

template <int n, int i, bool flag>
struct prime_checker : std::bool_constant<
	(n % i != 0) && prime_checker<n, i + 1, i * i <= n>::value
> {};

template <int n, int i>
struct prime_checker<n, i, false> : std::true_type {};

template <int n, int i>
struct is_prime_impl : prime_checker<n, i, (i * i <= n)> {};

template <int n>
struct is_prime : is_prime_impl<n, 2> {};

template <>
struct is_prime<1> : std::false_type {};

template <int n>
struct next_prime_impl : std::conditional_t<
	is_prime<n>::value,
	std::integral_constant<int, n>,
	next_prime_impl<n + 1>
> {};

template <int n>
struct next_prime : next_prime_impl<n + 1> {};

template <int k>
struct kth_prime : next_prime<kth_prime<k - 1>::value> {};

template <>
struct kth_prime<0> : std::integral_constant<int, 1> {};

static_assert(is_prime<1>::value == false, "1 is not prime");
static_assert(is_prime<2>::value == true, "2 is prime");
static_assert(is_prime<3>::value == true, "3 is prime");
static_assert(is_prime<4>::value == false, "4 is not prime");
static_assert(is_prime<5>::value == true, "5 is prime");

static_assert(kth_prime<0>::value == 1, "0th prime is 1");

static_assert(next_prime<1>::value == 2);
static_assert(next_prime<2>::value == 3);
static_assert(next_prime<3>::value == 5);

static_assert(kth_prime<1>::value == 2, "1st prime is 2");
static_assert(kth_prime<2>::value == 3, "2nd prime is 3");
static_assert(kth_prime<3>::value == 5, "3rd prime is 5");
static_assert(kth_prime<4>::value == 7, "4th prime is 7");
static_assert(kth_prime<5>::value == 11, "5th prime is 11");
static_assert(kth_prime<100>::value == 541, "100th prime is 541");
