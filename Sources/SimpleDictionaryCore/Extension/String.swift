//
//  String.swift
//  alfred-eudic-workflow
//
//  Created by Hanley Lee on 2024/12/7.
//

// 扩展：将字符串分块
extension String {
    // FIXME: some words will lost because the boundary of chunk may break a completed word
    func splitConcurrently(chunkSize: Int = 10_000) async -> [String] {
        // 将输入字符串分割为小块
        let chunks = self.split(intoChunksOf: chunkSize)

        // 使用 Task 并发处理每个小块
        let results = await withTaskGroup(of: (Int, [String]).self) { group -> [String] in
            for (index, chunk) in chunks.enumerated() {
                group.addTask {
                    // this place can't use parameter from outside, otherwise will be very slow
                    (index, chunk.components(separatedBy: .newlines))
                }
            }

            // 收集所有结果（带索引）
            var combinedResults: [(Int, [String])] = []
            for await result in group {
                combinedResults.append(result)
            }

            // 按索引排序以确保顺序
            combinedResults.sort(by: { $0.0 < $1.0 })

            // 合并排序后的结果
            return combinedResults.flatMap { $0.1 }
        }

        return results
    }

    func split(intoChunksOf chunkSize: Int) -> [String] {
        var chunks: [String] = []
        var startIndex = self.startIndex

        while startIndex < self.endIndex {
            let endIndex = self.index(startIndex, offsetBy: chunkSize, limitedBy: self.endIndex) ?? self.endIndex
            let chunk = String(self[startIndex ..< endIndex])
            chunks.append(chunk)
            startIndex = endIndex
        }

        return chunks
    }
}
