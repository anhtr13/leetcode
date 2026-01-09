package main

import (
	"github.com/anhtr13/leetcode/go/algorithms/common"
	"github.com/anhtr13/leetcode/go/algorithms/searching"
	my_heap "github.com/anhtr13/leetcode/go/data-structures/heap"
	"github.com/anhtr13/leetcode/go/data-structures/set"
	"github.com/anhtr13/leetcode/go/data-structures/tree"
	"github.com/anhtr13/leetcode/go/utils"
)

/*
<========== Data Structures ==========>
*/

type MinHeap = my_heap.MinHeap

type MaxHeap = my_heap.MaxHeap

type PQItem = my_heap.PQItem

type PriorityQueue = my_heap.PriorityQueue

type DisjointSet = set.DisjointSet

var NewDisjointSet = set.NewDisjointSet

type BalanceBinaryTree = tree.BalanceBinaryTree

type RedBlackTree = tree.RedBlackTree

type AVLTree = tree.AVLTree

var NewRedBlackTree = tree.NewRedBlackTree

var NewAVLTree = tree.NewAVLTree

/*
 */

/*
<========== Algorithms ==========>
*/

var FindLPS = searching.FindLPS

var KmpSearch = searching.KmpSearch

var FindGCD = common.FindGCD

var FindLCM = common.FindLCM

var FindPrimeNums = common.FindPrimeNums

var FindDivisors = common.FindDivisors

var Abs = common.Abs

/*
 */

/*
<========== Utilities ==========>
*/

type ListNode = utils.ListNode

type TreeNode = utils.TreeNode

var NewLinkedList = utils.NewLinkedList

var LinkedListToArr = utils.LinkedListToArr

var NewTree = utils.NewTree

var SpreadTree = utils.SpreadTree
