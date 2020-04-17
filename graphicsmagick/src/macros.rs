macro_rules! types_enum_block {
    (
        $(#[$enum_docs:meta])*
        $name:ident;
        $(
            $(#[$docs:meta])*
            ($num:expr, $item:ident);
        )+
    ) => {
        $(#[$enum_docs])*
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $name {
            $(
                $(#[$docs])*
                $item,
            )+
        }

        impl From<u32> for $name {
            fn from(x: u32) -> Self {
                $(
                    $(#[$docs])*
                    {
                        if x == $num {
                            return $name::$item;
                        }
                    }
                )+
                unreachable!()
            }
        }

        impl From<$name> for u32 {
            fn from(i: $name) -> u32 {
                match i {
                $(
                    $(#[$docs])*
                    $name::$item => $num,
                )+
                }
            }
        }
    }
}
