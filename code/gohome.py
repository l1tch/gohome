def countOrd(Uarr):
    leng = Uarr[0]+1                                                                     
    Oarr = [None] * (leng)                  
    
    for i in Uarr:                          
        if leng < i+1:                      
            nLen = (i+1) - leng
            leng = i+1
            Oarr = [*Oarr, * [None] * nLen]
        if Oarr[i] is None:
            Oarr[i] = 1
        else:
            Oarr[i] += 1      
    lst=[]
    for indx, val in enumerate(Oarr):
        if val != None:
            lst+=[indx] * val
    return lst
