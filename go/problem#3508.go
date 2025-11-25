package main

import "fmt"

type Pack struct {
	Source int
	Dest   int
	Time   int
	Id     int
}

type Router struct {
	Limit       int
	Size        int
	FirstPackId int
	CurPackId   int
	PackIdMap   map[int]*Pack
	DestMap     map[int]*[]*Pack
	Existed     map[string]bool
}

func NewRouter(memoryLimit int) Router {
	return Router{
		Limit:       memoryLimit,
		Size:        0,
		FirstPackId: 0,
		CurPackId:   -1,
		PackIdMap:   map[int]*Pack{},
		DestMap:     map[int]*[]*Pack{},
		Existed:     map[string]bool{},
	}
}

func (this *Router) AddPacket(source int, destination int, timestamp int) bool {
	key := fmt.Sprintf("%d:%d:%d", source, destination, timestamp)
	if this.Existed[key] {
		return false
	}
	this.CurPackId += 1
	pack := Pack{
		Source: source,
		Dest:   destination,
		Time:   timestamp,
		Id:     this.CurPackId,
	}
	if this.Size == this.Limit {
		this.ForwardPacket()
	}
	if this.DestMap[pack.Dest] == nil {
		this.DestMap[pack.Dest] = &[]*Pack{}
	}
	this.PackIdMap[pack.Id] = &pack
	*this.DestMap[pack.Dest] = append(*this.DestMap[pack.Dest], &pack)
	this.Existed[key] = true
	this.Size++
	return true
}

func (this *Router) ForwardPacket() []int {
	if this.Size == 0 {
		return []int{}
	}
	pack := this.PackIdMap[this.FirstPackId]
	dest := this.DestMap[pack.Dest]
	*dest = (*dest)[1:]
	key := fmt.Sprintf("%d:%d:%d", pack.Source, pack.Dest, pack.Time)
	this.Existed[key] = false
	this.PackIdMap[pack.Id] = nil
	this.Size--
	this.FirstPackId++
	return []int{pack.Source, pack.Dest, pack.Time}
}

func (this *Router) GetCount(destination int, startTime int, endTime int) int {
	if this.DestMap[destination] == nil {
		return 0
	}
	dest := *(this.DestMap[destination])
	if len(dest) == 0 {
		return 0
	}
	first_idx, second_idx := -1, -1
	l, r := 0, len(dest)-1
	for l < r {
		m := (l + r) / 2
		if dest[m].Time < startTime {
			l = m + 1
		} else {
			r = m
		}
	}
	if dest[l].Time < startTime {
		return 0
	}
	first_idx = l
	l, r = 0, len(dest)-1
	for l < r {
		m := (l + r + 1) / 2
		if dest[m].Time > endTime {
			r = m - 1
		} else {
			l = m
		}
	}
	if dest[r].Time > endTime {
		return 0
	}
	second_idx = r
	return second_idx - first_idx + 1
}

/**
 * Your Router object will be instantiated and called as such:
 * obj := Constructor(memoryLimit);
 * param_1 := obj.AddPacket(source,destination,timestamp);
 * param_2 := obj.ForwardPacket();
 * param_3 := obj.GetCount(destination,startTime,endTime);
 */
