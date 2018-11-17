// You can edit this code!
// Click here and start typing.
package main

import "fmt"

func main() {
    var a[100000000] float64
    for i := 0; i < 100000000; i++{
        a[i] = float64(i % 3)
    }
    for i := 0; i < 5; i++ {
        fmt.Println(a[i])
    }
}
