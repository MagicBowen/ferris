use resource::*;

#[test]
#[cfg(feature = "singleton_local_thread")]
fn test_in_single_thread() {
    for i in 0..10 {
        let pid = i as u32;
        config_process(pid).unwrap();

        #[cfg(feature = "resource_cpu")]
        config_allocation(pid, 3, ResourceType::CPU, 4).unwrap();

        #[cfg(feature = "resource_memory")]
        config_allocation(pid, 2, ResourceType::Memory, 2048).unwrap();

        #[cfg(feature = "resource_storage")]
        config_allocation(pid, 14, ResourceType::Storage, 100).unwrap();
    }

    for i in 0..10 {
        let pid = i as u32;
        match compute_process(pid).unwrap() {
            (cost, penalty) => {
                assert!(cost >= 0);
                assert!(penalty >= 0);
            }
        }
    }

    let result = compute_all_concurrent();

    assert_eq!(result.len(), 10);

    result.iter().for_each(|(_, cost, penalty)| {
        assert_eq!(*cost, 5856);
        assert_eq!(*penalty, 1);
    });
}

#[test]
#[cfg(not(feature = "singleton_local_thread"))]
fn test_in_multiple_threads() {
    for i in 0..10 {
        let pid = i as u32;
        config_process(pid).unwrap();
    }

    #[cfg(feature = "resource_cpu")]
    let handles1: Vec<_> = (0..10)
        .map(|i| {
            std::thread::spawn(move || {
                let pid = i as u32;
                config_allocation(pid, 3, ResourceType::CPU, 4).unwrap();
            })
        })
        .collect();

    #[cfg(feature = "resource_memory")]
    let handles2: Vec<_> = (0..10)
        .map(|i| {
            std::thread::spawn(move || {
                let pid = i as u32;
                config_allocation(pid, 2, ResourceType::Memory, 2048).unwrap();
            })
        })
        .collect();

    #[cfg(feature = "resource_storage")]
    let handles3: Vec<_> = (0..10)
        .map(|i| {
            std::thread::spawn(move || {
                let pid = i as u32;
                config_allocation(pid, 14, ResourceType::Storage, 100).unwrap();
            })
        })
        .collect();

    let handles4: Vec<_> = (0..10)
        .map(|i| {
            std::thread::spawn(move || {
                let pid = i as u32;
                match compute_process(pid).unwrap() {
                    (cost, penalty) => {
                        assert!(cost >= 0);
                        assert!(penalty >= 0);
                    }
                }
            })
        })
        .collect();

    #[cfg(feature = "resource_cpu")]
    for handle in handles1 {
        handle.join().unwrap();
    }

    #[cfg(feature = "resource_memory")]
    for handle in handles2 {
        handle.join().unwrap();
    }

    #[cfg(feature = "resource_storage")]
    for handle in handles3 {
        handle.join().unwrap();
    }

    for handle in handles4 {
        handle.join().unwrap();
    }

    let result = compute_all_concurrent();
    assert_eq!(result.len(), 10);

    result.iter().for_each(|(_, cost, penalty)| {
        assert_eq!(*cost, 5856);
        assert_eq!(*penalty, 1);
    });
}
