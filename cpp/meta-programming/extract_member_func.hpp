#include <cassert>
#include <functional>
#include <type_traits>

template <typename Ret, typename Class, typename... Args>
auto extract_member_func(Ret (Class::*func)(Args...), Class* obj) {
	return [obj, func](Args... args) -> Ret {
		return (obj->*func)(args...);
	};
}

class TestClass {
public:
	int add(int a, int b) {
		return a + b;
	}

	TestClass* get_this() {
		return this;
	}
};

void test() {
	TestClass* obj = new TestClass();
	auto add = extract_member_func(&TestClass::add, obj);
	auto get_this = extract_member_func(&TestClass::get_this, obj);
	
	assert(add(1, 2) == 3);
	assert(get_this() == obj);
}
