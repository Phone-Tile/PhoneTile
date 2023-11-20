//
//  init.c
//
//  Created by Emmanuel Mera on 16/11/2023.
//

// network specific lib
#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <string.h>

#include <stdbool.h>

#include "network.h"

#include "init.h"

#define PORT 8080

static int sockfd = 0;
static uint16_t session = 0;
static uint16_t status = DISCONNECTED;

_Bool handhsake(void) {
    char serMsg[] = {0, 1, 0, 0, 0, 0, 0, 0};
    send(sockfd, serMsg, sizeof(serMsg), 0);
    
    recv(sockfd, serMsg, sizeof(serMsg), 0);
    
    if (serMsg[1] == 1) {
        session = ((uint16_t)serMsg[4] << 8) + serMsg[5];
        printf("Session tocken ; %u\n", session);
        status = CONNECTED;
        return true;
    }
    return false;
}

_Bool NETconnect(void) {
    int connfd;
    struct sockaddr_in servaddr, cli;
 
    // socket create and verification
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd == -1) {
        printf("socket creation failed...\n");
        return false;
    }
    else
        printf("Socket successfully created..\n");
    bzero(&servaddr, sizeof(servaddr));
    
    // assign IP, PORT
    servaddr.sin_family = AF_INET;
    servaddr.sin_addr.s_addr = inet_addr("127.0.0.1");
    servaddr.sin_port = htons(PORT);
 
    // connect the client socket to server socket
    if (connect(sockfd, (struct sockaddr*)&servaddr, sizeof(servaddr)) != 0) {
        printf("connection with the server failed...\n");
        return false;
    }
    else
        printf("connected to the server..\n");
    
    return handhsake();
}

void NETdisconnect(void) {
    close(sockfd);
}

uint16_t NETgetStatus(void) {
    return status;
}
