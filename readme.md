<h1>Toga is a proc macro library for writing clean ergonomic rust</h1>

```rs
    trait Vehicle<T> {
        fn model(&self) -> T;
    }

    trait Tank {
        fn hit_points(&self) -> u8 {
            200
        }
    }

    trait Fast {
        fn speed(&self) -> u8;
    }


    struct Car {}

    toga::blockset! {
        impl<T: Default> Car;

        pub fn my_inherent_method(&self) {}

        Vehicle<T> {
            fn model(&self) -> T {
                T::default()
            }
        }

        Tank {}

        Fast {
            fn speed(&self) -> u8 {
                100
            }
        }
    }
```

## Still Work in Progress
