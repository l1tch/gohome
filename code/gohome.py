def countOrd(Uarr):
    leng = Uarr[0]+1                                                                     
    Oarr = [0] * (leng)                  
    
    for i in Uarr:                          
        if leng < i+1:                      
            nLen = (i+1) - leng
            leng = i+1
            Oarr = [*Oarr, * [0] * nLen]
        Oarr[i] += 1      
    lst=[]
    for indx, val in enumerate(Oarr):
        if val:
            lst+=[indx] * val
    return lst
