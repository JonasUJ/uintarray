use uintarray::UintArray;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ua = UintArray::new::<char>();

        assert_eq!(5, ua.0);
    }

    #[test]
    fn test_new_size() {
        let ua = UintArray::new_size(4);

        assert_eq!(2, ua.0);
    }

    #[test]
    fn test_from() {
        let ua = UintArray::from(69420);
        assert_eq!(16, ua.size());
    }

    #[test]
    #[should_panic]
    fn test_from_len_exceeds_cap() {
        UintArray::from(69421);
    }

    #[test]
    fn test_size() {
        let ua = UintArray(69420);
        assert_eq!(16, ua.size());
    }

    #[test]
    fn test_cap() {
        let ua = UintArray(69420);
        assert_eq!(7, ua.cap());
    }

    #[test]
    #[should_panic]
    fn test_size_big_panic() {
        UintArray::new_size(128);
    }

    #[test]
    #[should_panic]
    fn test_size_power_of_two_panic() {
        UintArray::new_size(15);
    }

    #[test]
    fn test_at() {
        // 524_314 = [0, 0, 8]
        let ua = UintArray(524_314);
        assert_eq!(Some(8), ua.at(2));
    }

    #[test]
    fn test_at_out_of_bounds() {
        let ua = UintArray(524_314);
        assert_eq!(None, ua.at(3));
    }

    #[test]
    fn test_len() {
        let ua = UintArray(524_314);
        assert_eq!(3, ua.len());
    }

    #[test]
    fn test_append() {
        let ua = UintArray(524_314);
        assert_eq!(4_718_626, ua.append(4).0);
    }

    #[test]
    #[should_panic]
    fn test_append_exceed_capacity() {
        let ua = UintArray::new::<u64>();

        // ua.cap() == 1
        ua.append(0).append(0);
    }

    #[test]
    #[should_panic]
    fn test_append_does_not_fit() {
        let ua = UintArray(524_314);

        // ua.size() == 4
        ua.append(16);
    }

    #[test]
    fn test_insert() {
        let ua = UintArray(524_314);
        assert_eq!(8_650_786, ua.insert(2, 4).0);
    }

    #[test]
    #[should_panic]
    fn test_insert_exceed_capacity() {
        let ua = UintArray::new::<u64>();

        // ua.cap() == 1
        ua.append(0).insert(0, 0);
    }

    #[test]
    fn test_extend() {
        let ua = UintArray(524_314);
        assert_eq!(18_020_302_906, ua.extend(1..5).0);
    }

    #[test]
    #[should_panic]
    fn test_extend_exceed_capacity() {
        let ua = UintArray(524_314);
        ua.extend((0..15).cycle().take(30));
    }

    #[test]
    #[should_panic]
    fn test_extend_beyond_capacity() {
        let ua = UintArray(524_314);
        ua.extend(0..100);
    }

    #[test]
    #[should_panic]
    fn test_extend_does_not_fit() {
        let ua = UintArray(524_314);
        ua.extend(16..);
    }

    #[test]
    fn test_clear() {
        let ua = UintArray(524_314);
        assert_eq!(2, ua.clear().0);
    }

    #[test]
    fn test_remove() {
        let ua = UintArray(524_314);
        assert_eq!(524_314, ua.remove(2).0);
        assert_eq!(32_786, ua.remove(0).0);
    }

    #[test]
    fn test_pop() {
        let ua = UintArray(524_314);
        let (ua, item) = ua.pop(1);
        assert_eq!(Some(0), item);
        assert_eq!(32_786, ua.0);
        assert_eq!(2, ua.len());

        let (ua, item) = ua.pop(2);
        assert_eq!(None, item);
        assert_eq!(32_786, ua.0);
        assert_eq!(2, ua.len());
    }

    #[test]
    fn test_index() {
        let ua = UintArray(524_314);
        assert_eq!(Some(2), ua.index(8));
        assert_eq!(None, ua.index(2));
    }

    #[test]
    fn test_count() {
        let ua = UintArray(524_314);
        assert_eq!(2, ua.count(0));
        assert_eq!(0, ua.count(2));
    }

    #[test]
    fn test_aggregate() {
        let ua = UintArray(524_314);
        assert_eq!(8, ua.aggregate(|x| x));
    }

    #[test]
    fn test_iterator() {
        // 1, 2, 3, 4
        let ua = UintArray(4_399_394);
        let mut i = 1;
        for u in ua {
            assert_eq!(i, u);
            i += 1;
        }
    }

    #[test]
    fn test_format() {
        let ua = UintArray(293399018589609169090056132135457263858);
        assert_eq!(ua.format(), "1101 1100 1011 1010 1001 1000 0111 0110\n0101 0100 0011 0010 0001 0000 1111 1110\n1101 1100 1011 1010 1001 1000 0111 0110\n0101 0100 0011 0010 0001 0000 1111 0010\n");
    }
}
