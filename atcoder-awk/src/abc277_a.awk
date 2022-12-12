{
    if(NR==1)
    {
        n=$1
        x=$2
    }
    if(NR==2)
    {
        for(i=1;i<=n;i++)
        {
            if($i==x)
            {
                print i
                break;
            }
        }
    }
}
