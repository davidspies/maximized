#[cfg(test)]
mod test {
    use borsh::BorshSerialize;
    use maximized::{byte_len, Maximized};
    use maximized_macros::Maximized;

    #[derive(BorshSerialize, Maximized)]
    struct Unit;

    #[derive(BorshSerialize, Maximized)]
    struct UnitParens();

    #[derive(BorshSerialize, Maximized)]
    struct SingleUnnamedField(u64);

    #[derive(BorshSerialize, Maximized)]
    struct MultipleUnnamedFields(u64, bool, u8);

    #[derive(BorshSerialize, Maximized)]
    struct UnitRecord {}

    #[derive(BorshSerialize, Maximized)]
    struct SingleFieldRecord {
        field1: u64,
    }
    #[derive(BorshSerialize, Maximized)]
    struct MultiFieldRecord {
        field1: u64,
        field2: bool,
        field3: u8,
    }

    #[derive(BorshSerialize, Maximized)]
    enum UnitEnum {
        Unit,
    }

    #[derive(BorshSerialize, Maximized)]
    enum MultiEnum {
        Zero,
        One,
        Two,
    }

    #[derive(BorshSerialize, Maximized)]
    enum EnumWithParams {
        UnitVariant,
        RecordVariant { field1: u64, field2: bool },
        UnnamedVariant(u64, u64, MultiEnum),
    }

    // Using a macro rather than a function so if it fails the error location is the call-site
    macro_rules! test_byte_len {
        ($T: ty) => {
            assert_eq!(
                <$T as Maximized>::compute_size(),
                byte_len(&<$T as Maximized>::maximized())
            )
        };
    }

    #[test]
    fn correct_sizes() {
        test_byte_len!(Unit);
        test_byte_len!(UnitParens);
        test_byte_len!(SingleUnnamedField);
        test_byte_len!(MultipleUnnamedFields);
        test_byte_len!(SingleFieldRecord);
        test_byte_len!(MultiFieldRecord);
        test_byte_len!(UnitEnum);
        test_byte_len!(MultiEnum);
        test_byte_len!(EnumWithParams);
    }
}
