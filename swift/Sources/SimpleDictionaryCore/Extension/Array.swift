//
//  Array.swift
//  alfred-eudic-workflow
//
//  Created by Hanley Lee on 2024/12/7.
//

extension Array where Element == String {
    // 二分查找匹配的索引
    func binarySearchMatchPrefix(target: String) -> Int? {
        var low = 0
        var high = count
        let prefixLen = target.count

        while low < high {
            let mid = (low + high) / 2
            let midStrPrefix = self[mid].prefix(prefixLen)

            if midStrPrefix == target {
                high = mid // 继续向左搜索
            } else if midStrPrefix < target {
                low = mid + 1
            } else if midStrPrefix > target {
                high = mid
            }
        }

        return low
    }
}

