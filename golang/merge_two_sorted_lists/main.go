package main

func main() {

}

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	if list1 == nil {
		return list2
	} else if list2 == nil {
		return list1
	}

	if list1.Val <= list2.Val {
		return &ListNode{
			Val:  list1.Val,
			Next: mergeTwoLists(list1.Next, list2),
		}
	} else {
		return &ListNode{
			Val:  list2.Val,
			Next: mergeTwoLists(list1, list2.Next),
		}
	}
}

// Non Recursive
// func newNode(val int) *ListNode {
//     return &ListNode {
//         Val: val,
//         Next: nil,
//     }
// }

// func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
//     if list1 == nil {
//         return list2
//     } else if list2 == nil {
//         return list1
//     }

//     var result *ListNode = nil
//     if list1.Val <= list2.Val {
//         result = newNode(list1.Val)
//         list1 = list1.Next
//     } else {
//         result = newNode(list2.Val)
//         list2 = list2.Next
//     }
//     var resultPtr *ListNode = result

//     for list1 != nil && list2 != nil {
//         if list1.Val <= list2.Val {
//             resultPtr.Next = newNode(list1.Val)
//             list1 = list1.Next
//         } else {
//             resultPtr.Next = newNode(list2.Val)
//             list2 = list2.Next
//         }

//         resultPtr = resultPtr.Next
//     }

//     for list1 != nil {
//         resultPtr.Next = newNode(list1.Val)
//         resultPtr = resultPtr.Next
//         list1 = list1.Next
//     }
//     for list2 != nil {
//         resultPtr.Next = newNode(list2.Val)
//         resultPtr = resultPtr.Next
//         list2 = list2.Next
//     }

//     return result
// }
