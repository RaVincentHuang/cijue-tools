import requests
import os
from bs4 import BeautifulSoup

from tqdm import tqdm
# 本地导入
#from download_one_paper import getHTMLText, download_one_paper

def getHTMLText(url):
    try:
        r= requests.get(url, timeout=30)
        r.raise_for_status()
        r.encoding= r.apparent_encoding
        return r.text
    except:
        return "getHTMLText error!"

def get_paper_url_list(html):
    '''获取所有论文的下载地址
    '''
    paper_url_list= []

    soup= BeautifulSoup(html, 'html.parser')
    for content in soup.find_all('a'):
        url= content.get('href')
        #print(url)
        url_str = str(url)
        if (url_str.startswith('https://www.vldb.org/pvldb/')):
     #   if (url!=None) and (url[0:26]=='https://www.vldb.org/pvldb/'):
            paper_url_list.append(url)
    paper_url_list= list(set(paper_url_list)) # 去重
    return paper_url_list

def get_paper_bib_url_list(html):
    '''获取所有论文的下载地址
    '''
    paper_url_list= []

    soup= BeautifulSoup(html, 'html.parser')
    for content in soup.find_all('a'):
        url= content.get('href')
      #  print(url)
        url_str = str(url)
        if (url_str.endswith('view=bibtex')):
     #   if (url!=None) and (url[0:26]=='https://www.vldb.org/pvldb/'):
            paper_url_list.append(url)
    paper_url_list= list(set(paper_url_list)) # 去重
    return paper_url_list

def get_paper_bib(html):
    soup= BeautifulSoup(html, 'html.parser')
    for content in soup.find_all(name='pre'):
        return str(content.text)



        #url= content.get('href')
      #  print(url)
        #url_str = str(url)
        #if (url_str.endswith('view=bibtex')):
     #   if (url!=None) and (url[0:26]=='https://www.vldb.org/pvldb/'):
         #   paper_url_list.append(url)
    #paper_url_list= list(set(paper_url_list)) # 去重
    #return paper_url_list 


def get_paper_bib_from_dblp(url: str) -> str:
    print(url)
    html= getHTMLText(url)
    paper_bib_url_list= get_paper_bib_url_list(html) # 获取所有论文的下载地址
    output_str = ''
    for bib_url in tqdm(paper_bib_url_list, desc="Processing papers"):
        html= getHTMLText(bib_url)

        bib_str = get_paper_bib(html)

        while(not bib_str):
            html= getHTMLText(bib_url)
            bib_str = get_paper_bib(html)
        output_str = output_str + bib_str
    
    return output_str


if __name__ == "__main__":

    conf_list=[
        {
            'url':'https://dblp.org/db/conf/iclr/iclr2024.html',
            'year':'2024',
            'typ':'A',
            'conf':'ICLR'
        }
    ]
    for conf in conf_list:
        conf_url= conf['url'] # 获取会议的网站
        print(conf_url)
        html= getHTMLText(conf_url)
       # print(html)
        paper_bib_url_list= get_paper_bib_url_list(html) # 获取所有论文的下载地址
        #print(paper_url_list)

        totnum_list= len(paper_bib_url_list)
        print(totnum_list)

       # i=3
    output_str = ''
    for i in range(len(paper_bib_url_list)):
        print('\ndealing with %d/%d=%f%%' % (i+1, totnum_list, 100.0*(i+1)/totnum_list)) # 用来观察进度
        bib_url = paper_bib_url_list[i]
        print(bib_url)
        html= getHTMLText(bib_url)

        bib_str = get_paper_bib(html)

        while(type(bib_str) == type(None)):
            html= getHTMLText(bib_url)
            bib_str = get_paper_bib(html)
        output_str = output_str + bib_str
    
    # Opening a file
    file1 = open(conf_list[0]['conf']+'-'+ conf_list[0]['year']+'-'+'bib_file.bib', 'w')
#L = ["This is Delhi \n", "This is Paris \n", "This is London \n"]
#s = "Hello\n"

# Writing a string to file
    file1.write(output_str)

# Writing multiple strings
# at a time
#file1.writelines(L)

# Closing file
    file1.close()




'''
        for i in range(len(paper_url_list)):
            print('\ndealing with %d/%d=%f%%' % (i+1, totnum_list, 100.0*(i+1)/totnum_list)) # 用来观察进度
            paper_url= paper_url_list[i] # paper_url= 'https://doi.org/10.1145/3299869.3314037'
            url_download_link_str = str(paper_url)
            file_name =url_download_link_str.split('/')
            write_name = file_name[-1]

            response = requests.get(paper_url)
            pdf = open(conf_list[0]['conf'] + '-' + conf_list[0]['year']+ '-' +str(write_name)+".pdf", 'wb')
            pdf.write(response.content)
            pdf.close()
            print("File ", write_name, " downloaded")
        print("All PDF files downloaded")
'''

            #download_one_paper(paper_url, conf['year'], conf['typ'], conf['conf'])