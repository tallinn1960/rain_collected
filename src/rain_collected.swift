import Foundation
import Dispatch

// create a function to calculate the amount of rain collected
// mark function as exported to c
@_cdecl("rainCollected")
public func rainCollected(_ heights: UnsafePointer<Int64>, _ count: Int) -> UInt64 {
    var leftMax: Int64 = Int64.min
    var rightMax: Int64 = Int64.min
    var left = 0
    var right = count - 1
    var collected: UInt64 = 0

    while left < right {
        if heights[left] < heights[right] {
            if heights[left] > leftMax {
                leftMax = heights[left]
            } else {
                collected += UInt64(leftMax - heights[left])
            }
            left += 1
        } else {
            if heights[right] > rightMax {
                rightMax = heights[right]
            } else {
                collected += UInt64(rightMax - heights[right])
            }
            right -= 1
        }
    }

    return collected
}


