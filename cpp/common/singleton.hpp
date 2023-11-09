template <typename T>
class Singleton {
public:
	static T& instance() {
		static T instance;
		return instance;
	}

	static T* instance_ptr() {
		return &instance();
	}

	Singleton(const Singleton&) = delete;
	Singleton& operator=(const Singleton&) = delete;
	Singleton(Singleton&&) = delete;
};
