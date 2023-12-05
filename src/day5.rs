use core::{num, fmt};
use std::ops::Deref;


pub fn d5(text : Vec<&str>) -> i32 {

    let (_, startList) = text[0].split_once(": ").unwrap();
    let startSeeds = startList.split(" ").map(|s| s.trim().parse::<usize>().unwrap());

    let mut mappingType = 0;
    let mut maps : Vec<Vec<Vec<usize>>> = Vec::new();

    for i in 2..text.len() {
        let trimmed = text[i].trim();
        if (trimmed == "") {
            continue;
        }
        if (!trimmed.chars().next().unwrap().is_ascii_digit()) {
            mappingType += 1;
            continue;
        }
        while (maps.len() <= mappingType) {
            maps.push(Vec::new())
        }
        //println!("E{:?}E", trimmed.split(" ").collect::<Vec<&str>>());
        maps[mappingType].push(trimmed.split(" ").map(|s| s.parse::<usize>().unwrap()).collect());
    }
    println!("Mappings made");

    let result = startSeeds.map(|s| {
        let mut newVal = s;
        for m in maps.iter() {
            newVal = mapnum(m, newVal);
            //print!("V{} ", newVal);
        }
        //println!();
        newVal
    });
    //println!("{:?}", result.collect::<Vec<usize>>());
    //return 5;
    return result.min().unwrap() as i32;
}

fn mapnum(maps : &Vec<Vec<usize>>, val : usize) -> usize {
    for i in 0..(maps.len()) {
        //println!("S{}:{}", val, maps[i][1]);
        let diff : i64 = val as i64 - maps[i][1] as i64;
        if diff >= 0 && diff < maps[i][2] as i64 {
            return maps[i][0] + diff as usize
        }
    }
    return val;
}

pub fn d5p2(text : Vec<&str>) -> i32 {
    let (_, startListStr) = text[0].split_once(": ").unwrap();
    let startSeedsList = startListStr.split(" ").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut seedTrees : Vec<Tree> = Vec::new();
    for i in (0..startSeedsList.len()).step_by(2) {
        let mut t : Tree = Tree::new();
        t.setRoot(NumRange{lower: startSeedsList[i], range: startSeedsList[i + 1], offset: 0});
        seedTrees.push(t);
    }
    let mut mappingType = 0;
    let mut maps : Vec<Vec<Vec<usize>>> = Vec::new();
    for i in 2..text.len() {
        let trimmed = text[i].trim();
        if (trimmed == "") {
            continue;
        }
        if (!trimmed.chars().next().unwrap().is_ascii_digit()) {
            mappingType += 1;
            continue;
        }
        while (maps.len() <= mappingType) {
            maps.push(Vec::new())
        }
        //println!("E{:?}E", trimmed.split(" ").collect::<Vec<&str>>());
        maps[mappingType].push(trimmed.split(" ").map(|s| s.parse::<usize>().unwrap()).collect());
    }
    maps = maps[1..].to_vec(); //not sure why but ok
    //println!("Mappings made : {:?}", maps);

    //split
    for m in maps {
        for t in &mut seedTrees {
            let leafs = t.leafIndexs();
            for l in leafs {
                split_to_tree(&m, t, l);
                
            }
            
        }
    }

    

    return  seedTrees.iter().map(|t| t.min()).min().unwrap() as i32;
}

fn split_to_tree(mappings : &Vec<Vec<usize>>, tree : &mut Tree, workingIndex : usize) {
    let numrange = tree.valueAtIndex(workingIndex);
    //println!("E");
    let mut mappedRanges : Vec<NumRange> = Vec::new();
    newMapRange(mappings, numrange, &mut mappedRanges);
    //println!("W");
    for mr in mappedRanges {
        tree.addChildByIndex(mr, workingIndex);
    }
    // for i in 0..mappings.len() {
    //     if (mappings[i][1] <= effectiveLower && mappings[i][1] + mappings[i][2] > effectiveLower) {
    //         if (mappings[i][1] + mappings[i][2] >= (effectiveLower as i64 + numrange.range) as usize) {
    //             let newOffset : i64 = numrange.offset + (mappings[i][0] - mappings[i][1]) as i64;
    //             let newRange = NumRange{
    //                 lower: numrange.lower,
    //                 range: numrange.range,
    //                 offset: newOffset
    //             };
    //             tree.addChildByIndex(newRange, workingIndex);
    //         }
    //         let subSetLen = (effectiveLower as i64 + numrange.range) as usize - mappings[i][1] + mappings[i][2];
    //         let subSet1 = NumRange{lower: numrange.lower, range: subSetLen, offset: numrange.offset};
    //         let subSet2 = NumRange{lower: numrange.lower + subSetLen, range: numrange.range - subSet1.range, offset: numrange.offset};

    //     }
    // }
}

pub fn mapRange(mappings : &Vec<Vec<usize>>, numrange : NumRange) -> Vec<NumRange> {
    let mut result = Vec::new();
    let effectiveLower : usize = (numrange.lower + numrange.offset) as usize;
    let mut workingIndexInRange : usize = 0;
    let mut edited = true;
    while (workingIndexInRange < numrange.range as usize && edited) {
        edited = false;
        let loopEffectiveLower : usize = effectiveLower + workingIndexInRange;
        println!("{}-{:?}", mappings.len(), numrange);

        println!("E");
        for i in 0..mappings.len() {
            if (mappings[i][1] <= loopEffectiveLower && mappings[i][1] + mappings[i][2] > loopEffectiveLower) {
                // if (mappings[i][1] + mappings[i][2] >= (loopEffectiveLower as i64 + numrange.range) as usize) {
                //     let newOffset : i64 = numrange.offset + (mappings[i][0] - mappings[i][1]) as i64;
                //     let newRange = NumRange{
                //         lower: numrange.lower,
                //         range: numrange.range,
                //         offset: newOffset
                //     };
                //     result.push(newRange);
                //     println!("OKKKKK");
                //     return result;
                // }
                
                //let subSetLen = (loopEffectiveLower as i64 + numrange.range) as usize - mappings[i][1];// + mappings[i][2];
                let mut subSetLen = mappings[i][1] - (loopEffectiveLower - workingIndexInRange);
                if (subSetLen <= 0) {
                    subSetLen = numrange.range as usize - workingIndexInRange;
                }
                let subSet1 = NumRange{lower: numrange.lower + workingIndexInRange as i64, range: subSetLen as i64, offset: numrange.offset + (mappings[i][0] - mappings[i][1]) as i64};
                println!("SSL: {}", subSetLen);
                workingIndexInRange += subSetLen;
                //let subSet2 = NumRange{lower: numrange.lower + subSetLen as i64, range: numrange.range - subSet1.range, offset: numrange.offset};
                result.push(subSet1);
                edited = true;
            }
        }
        
    }
    if (workingIndexInRange < numrange.range as usize) {
        let leftoverLen = numrange.range as usize - workingIndexInRange;
        result.push(NumRange { lower: (numrange.lower + workingIndexInRange as i64), range: (leftoverLen as i64), offset: (numrange.offset) })
    }
    println!("RESULT:{:?}", result);
    return result;

}

pub fn newd5p2(text : Vec<&str>) -> i32 {
    let (_, startListStr) = text[0].split_once(": ").unwrap();
    let startSeedsList = startListStr.split(" ").map(|s| s.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut ranges : Vec<NumRange> = Vec::new();
    for i in (0..startSeedsList.len()).step_by(2) {
        ranges.push(NumRange{lower: startSeedsList[i], range: startSeedsList[i + 1], offset: 0});
    }
    let mut mappingType = 0;
    let mut maps : Vec<Vec<Vec<usize>>> = Vec::new();
    for i in 2..text.len() {
        let trimmed = text[i].trim();
        if (trimmed == "") {
            continue;
        }
        if (!trimmed.chars().next().unwrap().is_ascii_digit()) {
            mappingType += 1;
            continue;
        }
        while (maps.len() <= mappingType) {
            maps.push(Vec::new())
        }
        //println!("E{:?}E", trimmed.split(" ").collect::<Vec<&str>>());
        maps[mappingType].push(trimmed.split(" ").map(|s| s.parse::<usize>().unwrap()).collect());
    }
    maps = maps[1..].to_vec(); //not sure why but ok

    for m in maps {
        let rangesLen = ranges.len();
        for i in (0..rangesLen).rev() {
            let r = ranges.remove(i);
            let mut newRanges : Vec<NumRange> = Vec::new();
            newMapRange(&m, r, &mut newRanges);
            ranges.append(&mut newRanges);
        }
        //println!("E{:?}", ranges);
    }
    //println!("\n\n\n");
    //println!("{:?}", ranges);
    return ranges.iter().map(|r| r.lower + r.offset).min().unwrap() as i32;

}

pub fn newMapRange(mappings : &Vec<Vec<usize>>, numrange : NumRange, results : &mut Vec<NumRange>) {
    let effectiveLower : usize = (numrange.lower + numrange.offset) as usize;
    let effectiveUpper : usize = (numrange.lower + numrange.offset + numrange.range - 1) as usize;
    let mut editedSomthing = false;
        for m in mappings {
            let mapLower = m[1];
            let mapUpper = mapLower + m[2] - 1;
            if (effectiveLower <= mapUpper && effectiveUpper >= mapLower) {
                //some crossover
                editedSomthing = true;
                let crossOverStart = effectiveLower.max(mapLower);
                let crossOverEnd = effectiveUpper.min(mapUpper);
                //println!("{},{}", crossOverStart, crossOverEnd);
                //return;
                let beforeLen = crossOverStart - effectiveLower;
                let afterLen = effectiveUpper - crossOverEnd;
                let newOffset = numrange.offset + m[0] as i64 - mapLower as i64;
                results.push(NumRange{lower: crossOverStart as i64 - numrange.offset, range: (crossOverEnd - crossOverStart + 1) as i64, offset: (newOffset)});
                if (beforeLen > 0) {
                    newMapRange(mappings, NumRange { lower: (numrange.lower), range: (beforeLen as i64), offset: (numrange.offset) }, results)
                }
                if (afterLen > 0) {
                    newMapRange(mappings, NumRange { lower: (crossOverEnd as i64 - numrange.offset + 1), range: (afterLen as i64), offset: (numrange.offset) }, results)
                }
                
            }
        }
        if (!editedSomthing) {
            results.push(numrange);
        }
        
}

struct Tree {
    arr : Vec<TreeNode>,
}

impl Tree {
    pub fn new() -> Tree {
        return Tree { arr: (Vec::new()) }
    }

    // pub fn addChild(&mut self,  val : NumRange, path : impl Iterator<Item = usize>) -> Vec<usize> {
    //     let parentIndex = self.traverseReturnIndex(path);
    //     self.arr.push(TreeNode{val, children: Vec::new()});
    //     let childIndex = self.arr.len() - 1;
    //     self.arr[parentIndex].children.push(childIndex);
    //     let mut childPath : Vec<usize> = path.collect::<Vec<usize>>();
    //     childPath.push(self.arr[parentIndex].children.len() - 1);
    //     return childPath;
    // }
    pub fn addChildByIndex(&mut self,  val : NumRange, index : usize) {
        let parentIndex = index;
        self.arr.push(TreeNode{val, children: Vec::new()});
        let childIndex = self.arr.len() - 1;
        self.arr[parentIndex].children.push(childIndex);
    }
    pub fn getChildrenByIndex(&self, index : usize) -> &Vec<usize> {
        &self.arr[index].children
    }
    pub fn setRoot(&mut self, val : NumRange) {
        self.arr = Vec::new();
        self.arr.push(TreeNode { val: (val), children: (Vec::new()) })
    }
    pub fn valueAtIndex(&self, index : usize) -> NumRange {
        self.arr[index].val.clone()
    }

    pub fn traverseReturnIndex(&self, mut path : impl Iterator<Item = usize>) -> usize {
        let mut index : usize = 0;
        while let Some(pathChoice) = path.next() {
            index = self.arr[index].children[pathChoice];
        }
        index
    }
    pub fn traverseReturnTreeNode(&self, mut path : impl Iterator<Item = usize>) -> &TreeNode {
        &self.arr[self.traverseReturnIndex(path)]
    }
    pub fn traverseReturnValue(&self, mut path : impl Iterator<Item = usize>) -> &NumRange {
        &self.arr[self.traverseReturnIndex(path)].val
    }
    pub fn leafIndexs(&self) -> Vec<usize> {
        self.arr.iter().enumerate().filter_map(|(i, tn)| {
            if tn.children.len() == 0 {
                return Some(i);
            }
            return None;
        }).collect()
    }

    pub fn min(&self) -> i64 {
        self.arr.iter().map(|tn| tn.val.lower + tn.val.offset).min().unwrap()
    }
}

struct TreeNode {
    pub val : NumRange,
    pub children : Vec<usize>
}

pub struct NumRange {
    pub lower : i64,
    pub range : i64,
    pub offset : i64
}

impl NumRange {
    pub fn clone(&self) -> NumRange {
        NumRange { lower: (self.lower), range: (self.range), offset: (self.offset) }
    
    }
}

impl fmt::Debug for NumRange {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NumRange")
            .field("lower", &self.lower)
            .field("range", &self.range)
            .field("offset", &self.offset)
            .finish()
            
    }
}