package main

import (
	"container/heap"
	"slices"
)

type Item1912 struct {
	Shop   int
	Movie  int
	Price  int
	Rented bool
	Idx    int
}

type RentedItems1912 []*Item1912

func (list RentedItems1912) Len() int { return len(list) }

func (list RentedItems1912) Less(i, j int) bool {
	if list[i].Price == list[j].Price {
		if list[i].Shop == list[j].Shop {
			return list[i].Movie < list[j].Movie
		}
		return list[i].Shop < list[j].Shop
	}
	return list[i].Price < list[j].Price
}

func (list RentedItems1912) Swap(i, j int) {
	list[i], list[j] = list[j], list[i]
	list[i].Idx = i
	list[j].Idx = j
}

func (list *RentedItems1912) Push(x any) {
	n := len(*list)
	item := x.(*Item1912)
	item.Idx = n
	item.Rented = true
	*list = append(*list, item)
}

func (list *RentedItems1912) Pop() any {
	old := *list
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.Idx = -1
	*list = old[0 : n-1]
	return item
}

type MovieRentingSystem struct {
	Movie_ListItem map[int][]*Item1912       // map[movie]*[]*Item
	MovieShop_Item map[int]map[int]*Item1912 // map[movie][shop]*Item
	RentedItem     *RentedItems1912
}

func Constructor(n int, entries [][]int) MovieRentingSystem {
	movie_listitem := map[int][]*Item1912{}
	movieshop_item := map[int]map[int]*Item1912{}
	rented_item := &RentedItems1912{}
	for _, e := range entries {
		if movie_listitem[e[1]] == nil {
			movie_listitem[e[1]] = []*Item1912{}
			movieshop_item[e[1]] = map[int]*Item1912{}
		}
		item := &Item1912{
			Shop:  e[0],
			Movie: e[1],
			Price: e[2],
		}
		movie_listitem[e[1]] = append(movie_listitem[e[1]], item)
		movieshop_item[e[1]][e[0]] = item
	}
	for _, list := range movie_listitem {
		slices.SortFunc(list, func(a, b *Item1912) int {
			if a.Price < b.Price {
				return -1
			}
			if a.Price > b.Price {
				return 1
			}
			if a.Shop < b.Shop {
				return -1
			}
			if a.Shop > b.Shop {
				return 1
			}
			return 0
		})
	}
	heap.Init(rented_item)
	return MovieRentingSystem{
		Movie_ListItem: movie_listitem,
		MovieShop_Item: movieshop_item,
		RentedItem:     rented_item,
	}
}

func (this *MovieRentingSystem) Search(movie int) []int {
	res := []int{}
	list := this.Movie_ListItem[movie]
	i := 0
	for i < len(list) && len(res) < 5 {
		if !list[i].Rented {
			res = append(res, list[i].Shop)
		}
		i++
	}
	return res
}

func (this *MovieRentingSystem) Rent(shop int, movie int) {
	item := this.MovieShop_Item[movie][shop]
	item.Rented = true
	heap.Push(this.RentedItem, item)
}

func (this *MovieRentingSystem) Drop(shop int, movie int) {
	item := this.MovieShop_Item[movie][shop]
	p := item.Price
	item.Price = -1
	heap.Fix(this.RentedItem, item.Idx)
	heap.Pop(this.RentedItem)
	item.Price = p
	item.Rented = false
}

func (this *MovieRentingSystem) Report() [][]int {
	res := [][]int{}
	backup := []*Item1912{}
	i := 0
	for i < 5 && this.RentedItem.Len() > 0 {
		item := heap.Pop(this.RentedItem).(*Item1912)
		backup = append(backup, item)
		res = append(res, []int{item.Shop, item.Movie})
		i++
	}
	for _, item := range backup {
		heap.Push(this.RentedItem, item)
	}
	return res
}

/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * obj := Constructor(n, entries);
 * param_1 := obj.Search(movie);
 * obj.Rent(shop,movie);
 * obj.Drop(shop,movie);
 * param_4 := obj.Report();
 */
