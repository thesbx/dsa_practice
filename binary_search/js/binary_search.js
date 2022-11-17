function createArray(length) {
    const array = new Array(length)

    for (var i = 0; i < length; i++) {
        array[i] = i + 1; //This populates the array.  +1 is necessary because arrays are 0 index based and you want to store 1-100 in it, NOT 0-99.
    }
    return array
}

function binary_search(data, item) {
    let low = 0
    let high = data.length - 1
    let st = performance.now()

    while (low <= high) {
        let mid = Math.floor((low + high) / 2)
        let guess = data[mid]

        if (guess == item) {
            let et = performance.now()
            console.log(`Result: ${mid}`)
            console.log(`Binary Search Exec: ${et - st}ms`)
            return
        }
        if (guess > item) {
            high = mid - 1
        }
        else {
            low = mid + 1
        }
    }
}

function linear_search(data, item) {
    let st = performance.now()
    for (let i = 0; i < data.length; i++) {
        if (data[i] == item) {
            let et = performance.now()
            console.log(`Result: ${data[i] - 1}`)
            console.log(`Linear Search Exec: ${ et - st}ms`)
        }
    }
}

const args = process.argv

if (args[2]) {
    binary_search(createArray(args[2]), args[2])
    console.warn("---------------------")
    linear_search(createArray(args[2]), args[2])
} else {
    console.log("No argument provided, please enter a number")
}
