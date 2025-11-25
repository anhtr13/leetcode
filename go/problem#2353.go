package main

import "container/heap"

type FoodItem struct {
	Food    string
	Cuisine string
	Rating  int
	Index   int
}

type CuisineQueue []*FoodItem

func (cq CuisineQueue) Len() int { return len(cq) }

func (cq CuisineQueue) Less(i, j int) bool {
	if cq[i].Rating == cq[j].Rating {
		return cq[i].Food < cq[j].Food
	}
	return cq[i].Rating > cq[j].Rating
}

func (cq CuisineQueue) Swap(i, j int) {
	cq[i], cq[j] = cq[j], cq[i]
	cq[i].Index = i
	cq[j].Index = j
}

func (cq *CuisineQueue) Push(x any) {
	n := len(*cq)
	item := x.(*FoodItem)
	item.Index = n
	*cq = append(*cq, item)
}

func (cq *CuisineQueue) Pop() any {
	old := *cq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.Index = -1
	*cq = old[0 : n-1]
	return item
}

type FoodRatings struct {
	FoodMap    map[string]*FoodItem
	CuisineMap map[string]*CuisineQueue
}

func NewFoodRating(foods []string, cuisines []string, ratings []int) FoodRatings {
	fr := FoodRatings{
		FoodMap:    map[string]*FoodItem{},
		CuisineMap: map[string]*CuisineQueue{},
	}
	for i := range foods {
		f := foods[i]
		c := cuisines[i]
		r := ratings[i]
		fitem := &FoodItem{
			Food:    f,
			Cuisine: c,
			Rating:  r,
		}
		cq := fr.CuisineMap[c]
		if cq == nil {
			cq = &CuisineQueue{}
			heap.Init(cq)
			fr.CuisineMap[c] = cq
		}
		heap.Push(cq, fitem)
		fr.FoodMap[f] = fitem
	}
	return fr
}

func (this *FoodRatings) ChangeRating(food string, newRating int) {
	fitem := this.FoodMap[food]
	cq := this.CuisineMap[fitem.Cuisine]
	fitem.Rating = newRating
	heap.Fix(cq, fitem.Index)
}

func (this *FoodRatings) HighestRated(cuisine string) string {
	fitem := (*(this.CuisineMap[cuisine]))[0]
	return fitem.Food
}
